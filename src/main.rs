#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod models;
mod public_holidays;
mod routes;

use routes::*;

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

