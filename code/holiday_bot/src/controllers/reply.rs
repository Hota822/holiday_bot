use serde_json::value::Value;
use crate::line::{bot, utils, messages, events};
use crate::models::line::*;
use super::get_secret_data;

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

fn create_msg_for_text(_data: Value) -> messages::LineMessage {
    // オウム返し
    // let text = &data["events"][0]["message"]["text"].as_str().unwrap();
    // let message = messages::LineMessage::create_text("",text);

    if true {
    // if db.new_user {
        // db.add_user_id
        messages::LineMessage::create_text("", "お前のことはおぼえたぞ")
    } else {
        let next_holiday = "sample"; // db.holiday.first.get();
        let mut message = "next holiday is ".to_string();
        message.push_str(next_holiday);
        if true {
        // if let Some(day) db.personal_holiday.get() {
            let day = "sample day"; // if implemented this scope, delete this line
            message.push_str("\nnext your special day is ");
            message.push_str(&day);
            return messages::LineMessage::create_text("", &message)
        }
        messages::LineMessage::create_text("", &message)
    }
}

fn not_supported(_data: Value) -> messages::LineMessage {
    let message = messages::LineMessage::create_text("", "未対応");
    message
}