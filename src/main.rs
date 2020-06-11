#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate line_messaging_api_rust;

mod models;
mod public_holidays;
mod routes;
// mod line_messaging_api_rocket;

// pub use line_messaging_api_rocket::rocket_line::*;
// use line_messaging_api_rust::*;
use routes::*;

fn main() {
    rocket::ignite().mount("/", routes![index])
                    .mount("/", routes![webhook]).launch();
}

