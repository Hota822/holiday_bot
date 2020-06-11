use crate::models::*;
use crate::public_holidays::*;
// use line_messaging_api_rust;
// use line_messaging_api_rocket;

#[get("/")]
pub fn index() -> &'static str {
    "Hello"
}

#[post("/callback", format = "application/json", data = "<body>")]
pub fn webhook(signature: Signature, body: Body) -> &'static str {
// fn webhook ( bd: String) -> &'static str {       // if you want to render request body, acitvate this line
    const BASE_URL: &str = "https://api.line.me/v2/bot/message/reply";

    println!("web hook has sent");
    println!("=======================================================");
    // println!("body: {}", body.name);
    // println!("signature: {}",signature.key );
    println!("=======================================================");
    println!("send http request");
    // send http request
    // let s = ureq::get("http://127.0.0.1:8000/").call().into_string().unwrap();
    // println!("req {}", s);
    println!("=======================================================");

    "bbbbb"

}
