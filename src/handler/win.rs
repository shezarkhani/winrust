use std::ops::Deref;

use iron;
use iron::prelude::*;
use iron::Handler;

use params::{Params, Value};

use middleware::redis::RedisReqExt;
use service::counter;

pub struct WinHandler {}
impl WinHandler {
    pub fn new() -> WinHandler {
        WinHandler{}
    }
}

impl Handler for WinHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let connection = req.redis();

        let map = req.get_ref::<Params>().unwrap();

        match map.get("event_id") {
            Some(&Value::String(ref name)) => {
                counter::count(connection.deref(), String::from("event_id"), name.to_owned());
            }

            _ => {}
        }

        Ok(Response::with((iron::status::Ok, String::from("OK"))))
    }
}
