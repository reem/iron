extern crate iron;
extern crate env_logger;

use iron::prelude::*;
use iron::status;

fn main() {
    env_logger::init().unwrap();
    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello world!")))
    }).http("localhost:3000").unwrap();
}

