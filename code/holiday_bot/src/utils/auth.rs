use std::env;


// change "SECRET" to "ACCESS_PATH" is required
const ACCESS_PATH: &'static str = "SECRET";
const CHANNEL_SECRET: &str = "CHANNEL_SECRET";
const CHANNEL_ACCESS_TOKEN: &str = "CHANNEL_ACCESS_TOKEN";

pub fn check_access_pass(access_pass: &'static str) -> Result<(), String> {
    let pass_in_env = match get_pass_in_env() {
        Ok(pass) => pass,
        Err(err) => return Err(format!("{:?}", err)),
    };
    if pass_in_env == access_pass {
        Ok(())
    } else {
        return Err("mismatched access pass".to_string())
    }
}

pub fn check_secret_data(secret: &'static str, token: &'static str) -> Result<(), String> {
    use crate::line::bot;

    let (secret_in_env, token_in_env) = match get_secret_data() {
        Ok((secret, token)) => (secret, token),
        Err(err) => return Err(format!("{:?}", err)),
    };
    let bot = bot::LineBot::new(&secret_in_env, &token_in_env);
    if !bot.check_signature(secret, token) { return Err("mismatched channel secret data".to_string()) }
    Ok(())
}

fn get_pass_in_env() -> Result<String, env::VarError> {
    let access_pass = env::var(ACCESS_PATH)?;
    Ok(access_pass)
}

fn get_secret_data() -> Result<(String, String), env::VarError> {
    let secret = env::var(CHANNEL_SECRET)?;
    let token = env::var(CHANNEL_ACCESS_TOKEN)?;
    Ok((secret, token))
}