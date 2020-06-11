#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate line_messaging_api_rust as line;
extern crate serde_json;

mod models;
mod public_holidays;
mod routes;

use routes::*;

fn main() {
    rocket::ignite().mount("/", routes![index])
                    .mount("/", routes![webhook]).launch();
}

