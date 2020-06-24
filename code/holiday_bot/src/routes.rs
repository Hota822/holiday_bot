use crate::controllers;
use crate::models::*;

// pub const U: chrono::NaiveDate = chrono::naive::MIN_DATE;

// const BASE_URL: &str = "https://api.line.me/v2/bot/message/reply";
// const CHANNEL_SECRET: &str = "CHANNEL_SECRET";
// const CHANNEL_ACCESS_TOKEN: &str = "CHANNEL_ACCESS_TOKEN";

#[get("/")]
pub fn index() -> &'static str {
    "Hello holiday!\n"
}
#[post("/callback", format = "application/json", data = "<body>")]
pub fn webhook(signature: Signature, body: Body) -> Result<(), &'static str> {
    let function = controllers::reply::reply;
    controllers::reply::construct(signature, body, function)
}

// pub fn cron(signature: Signature, body: Body) -> Result<(), &'static str> {
#[post("/everyday", format = "application/json", data = "<body>")]
pub fn cron(signature: Secret, body: Body) -> &'static str {
    // println!("everyday received request.");
    controllers::push::construct(signature, body);
    "Push success!"
}

#[post("/test", format = "application/json", data = "<body>")]
pub fn test(header: HeaderTest, body: BodyTest)  {
    println!("Request header is:");
    println!("{:?}", header);
    println!();
    println!("Request body is:");
    println!("{:?}", body);
    println!();
}