use crate::line::{bot, utils, messages, events};
use crate::controllers;
use crate::models::*;
use crate::public_holidays::*;
use serde_json::value::Value;
use std::env;

// pub const U: chrono::NaiveDate = chrono::naive::MIN_DATE;

// const BASE_URL: &str = "https://api.line.me/v2/bot/message/reply";
// const CHANNEL_SECRET: &str = "CHANNEL_SECRET";
// const CHANNEL_ACCESS_TOKEN: &str = "CHANNEL_ACCESS_TOKEN";

#[get("/")]
pub fn index() -> &'static str {
    "Hello"
}
#[post("/callback", format = "application/json", data = "<body>")]
pub fn webhook(signature: Signature, body: Body) {
    let function = controllers::reply;
    controllers::construct(signature, body, function);
}
// #[post("/callback", format = "application/json", data = "<body>")]
// pub fn webhook(signature: Signature, body: Body) -> Result<(), env::VarError> {
    // println!("web hook has sent");
    // println!("body: {}", body.name);
    // println!("signature: {}",signature.key );
    // println!("=======================================================");
    // let secret = env::var(CHANNEL_SECRET)?;
    // let token = env::var(CHANNEL_ACCESS_TOKEN)?;
    // let bot = bot::LineBot::new(&secret, &token);

    // if bot.check_signature(&body.data, &signature.key)
    //     && utils::is_replyable(&body.get_data())
    // {
    //     let data: &str = &body.get_data();
    //     let receive_event: Value = serde_json::from_str(data).unwrap();
    //     let event: events::ReplyableEvent = utils::to_events(&body.get_data()).unwrap();
    //     if receive_event["events"][0]["message"]["type"] == "text"{
    //         let receive_text: &str = &receive_event["events"][0]["message"]["text"].as_str().unwrap();
    //         let message = messages::LineMessage::create_text("", receive_text);
    //         event.reply(vec![message], bot);
    //     }
    // }
    // Ok(())