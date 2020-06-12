#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
pub use line_messaging_api_rust as line;
pub use serde_json;

mod controllers;
mod models;
mod public_holidays;
mod routes;


use routes::*;

fn main() {
    rocket::ignite().mount("/", routes![index])
                    .mount("/", routes![webhook]).launch();
}

