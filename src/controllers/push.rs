use serde_json::value::Value;
// use crate::line::{bot, utils, messages, events};
use crate::line::{bot, messages};
use crate::models::*;
use crate::public_holidays as ph;
// use crate::utils::holiday;
use crate::utils::holiday as date;

use super::get_secret_data;

const SECRET: &str = "hota822ss";

pub fn construct(signature: Secret, body: Body) -> Result<(), &'static str>{
    // let (secret, _) = match get_secret_data() {
    //     Ok((secret, token)) => (secret, token),
    //     Err(_) => return Err("Failure"),
    // };
    let secret = std::env::var(SECRET).unwrap();
    if secret != signature.key { return Err("Failure") };
    push();
    Ok(())
}

fn push() {
    let (secret, token) = match get_secret_data() {
        Ok((secret, token)) => (secret, token),
        Err(_) => ("failure".to_string(), "failure".to_string() ),
    };
    let bot = bot::LineBot::new(&secret, &token);
    let tomorrow = date::tomorrow();
    let next_month = date::next_month();

    // let today = Local::today().naive_local();
    // get first data in database holiday
    // if first holiday == tomorrow  {
        // line push
    // }
    // if first holiday == next_month {
        // line push
    // }
    // get personal day in database personal_holiday
    // if first holiday is next day {

    // }
    // if first holiday is two weeks (or 1 month?) later {

    // }




    // let data: &str = &body.get_data();
    // let event: events::ReplyableEvent = utils::to_events(data).unwrap();
    // let data: Value = serde_json::from_str(data).unwrap();
    // let function: fn(Value) -> messages::LineMessage;
    // match &data["events"][0]["message"]["type"] {
        // Value::String(_text) => function = create_msg_for_text,
        // _ => function = not_supported,
    // };
    // let message = function(data);
    // event.reply(vec![message], bot);
}

fn create_msg_for_text(data: Value) -> messages::LineMessage {
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

fn not_supported(data: Value) -> messages::LineMessage {
    let message = messages::LineMessage::create_text("", "未対応");
    message
}
