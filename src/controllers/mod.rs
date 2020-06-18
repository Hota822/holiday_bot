pub mod reply;
pub mod push;
use std::env;

const CHANNEL_SECRET: &str = "CHANNEL_SECRET";
const CHANNEL_ACCESS_TOKEN: &str = "CHANNEL_ACCESS_TOKEN";

fn get_secret_data() -> Result<(String, String), env::VarError> {
    let secret = env::var(CHANNEL_SECRET)?;
    let token = env::var(CHANNEL_ACCESS_TOKEN)?;
    Ok((secret, token))
}

