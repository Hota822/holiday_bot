#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
// #[macro_use] extern crate serde_derive;
pub use line_messaging_api_rust as line;
pub use serde;
pub use serde_json;
pub use chrono;

// pub mod cors;
pub mod schema; 

mod controllers;
mod models;
mod test_models;
mod public_holidays;
mod routes;
mod utils;

use routes::*;

#[database("holiday_bot")]
pub struct DBConnection(diesel::MysqlConnection);

fn main() {
    rocket::ignite().mount("/", routes![index])
                    .mount("/", routes![webhook])
                    .mount("/", routes![cron])
                    .mount("/", routes![test])
                    .attach(DBConnection::fairing())
                    .launch();
}

