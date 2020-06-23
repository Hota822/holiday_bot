pub mod reply;
pub mod push;
use std::env;

const CHANNEL_SECRET: &str = "CHANNEL_SECRET";
const CHANNEL_ACCESS_TOKEN: &str = "CHANNEL_ACCESS_TOKEN";
const USER_ID: &str = "USER_ID";

fn get_secret_data() -> Result<(String, String), env::VarError> {
    let secret = env::var(CHANNEL_SECRET)?;
    let token = env::var(CHANNEL_ACCESS_TOKEN)?;
    Ok((secret, token))
}

fn get_user() -> Result<Vec<String>, env::VarError> {
    // get target user from db
    let user_ids = env::var(USER_ID)?;
    let user_ids = user_ids.split(",").map(|s| s.to_string()).collect::<Vec<String>>();
    Ok(user_ids)
}

