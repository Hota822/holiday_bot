use crate::controllers;
use crate::models::*;

// pub const U: chrono::NaiveDate = chrono::naive::MIN_DATE;

// const BASE_URL: &str = "https://api.line.me/v2/bot/message/reply";
// const CHANNEL_SECRET: &str = "CHANNEL_SECRET";
// const CHANNEL_ACCESS_TOKEN: &str = "CHANNEL_ACCESS_TOKEN";

#[get("/")]
pub fn index() -> &'static str {
    "Hello"
}
#[post("/callback", format = "application/json", data = "<body>")]
pub fn webhook(signature: Signature, body: Body) -> Result<(), &'static str> {
    let function = controllers::reply::reply;
    controllers::reply::construct(signature, body, function)
}

#[post("/everyday", format = "application/json", data = "<body>")]
// pub fn cron(signature: Signature, body: Body) -> Result<(), &'static str> {
pub fn cron(signature: Signature, body: Body) -> &'static str {
    // controllers::push::construct(signature, body)
    println!("everyday received request.");
    "aaaaaaaaaaaaaaaa"
}