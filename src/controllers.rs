use serde_json::value::Value;
use crate::line::{bot, utils, messages, events};
use crate::models::*;

// use std::error::Error;
use std::env;

const CHANNEL_SECRET: &str = "CHANNEL_SECRET";
const CHANNEL_ACCESS_TOKEN: &str = "CHANNEL_ACCESS_TOKEN";

pub fn construct(
    signature: Signature,
    body: Body,
    function: fn(Body, bot::LineBot) ) -> Result<(), &'static str>
{
    let (secret, token) = match get_secret_data() {
        Ok((secret, token)) => (secret, token),
        Err(_) => return Err("Failure"),
    };
    let bot = bot::LineBot::new(&secret, &token);
    if !bot.check_signature(&body.data, &signature.key) { return Err("Failure") }
    if !utils::is_replyable(&body.get_data()) { return Err("Failure") }
    function(body, bot);
    Ok(())
}

fn get_secret_data() -> Result<(String, String), env::VarError> {
    let secret = env::var(CHANNEL_SECRET)?;
    let token = env::var(CHANNEL_ACCESS_TOKEN)?;
    Ok((secret, token))
}

pub fn reply(body: Body, bot: bot::LineBot) {
    let data: &str = &body.get_data();
    let event: events::ReplyableEvent = utils::to_events(data).unwrap();
    let data: Value = serde_json::from_str(data).unwrap();
    let function: fn(Value) -> messages::LineMessage;
    match &data["events"][0]["message"]["type"] {
        Value::String(_text) => function = create_msg_for_text,
        _ => function = not_supported,
    };
    let message = function(data);
    event.reply(vec![message], bot);
}



fn create_msg_for_text(data: Value) -> messages::LineMessage {
    // let text = &data["events"][0]["message"]["text"].as_str().unwrap();
    // let message = messages::LineMessage::create_text("",text);
    let message = messages::LineMessage::create_text("", "お前のことはおぼえたぞ");
    message
}

fn not_supported(data: Value) -> messages::LineMessage {
    let message = messages::LineMessage::create_text("", "お前のことはおぼえたぞ");
    message
}