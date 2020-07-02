use rocket::Outcome;
use rocket::Outcome::*;
use rocket::request::{self, Request, FromRequest};
use rocket::http::Status;
use rocket::Data;
use rocket::data::{self, FromDataSimple};
use crate::line::bot::LineBot;

use std::env;
use std::io::Read;

pub struct Signature {
    pub key: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Signature {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Signature, ()> {
        let keys: Vec<_> = request.headers().get("X-Line-Signature").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        Outcome::Success(
            Signature { key: keys[0].to_string() }
        )
    }
}

pub struct Secret {
    pub key: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Secret {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Secret, ()> {
        let keys: Vec<_> = request.headers().get("secret").collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::BadRequest, ()));
        }

        Outcome::Success(
            Secret { key: keys[0].to_string() }
        )
    }
}

#[derive(Debug)]
pub struct HeaderTest {
    header: Vec<String>,
}

impl<'a, 'r> FromRequest<'a, 'r> for HeaderTest {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<HeaderTest, ()> {
        let mut vector: Vec<String> = Vec::new();
        for v in request.headers().iter() {
            vector.push(v.to_string());
        };
        Outcome::Success(
            HeaderTest { header: vector }
        )
    }
}

#[derive(Debug)]
pub struct BodyTest {
    data: String,
}

impl FromDataSimple for BodyTest {
    type Error = String;

    fn from_data(_: &Request, data: Data) -> data::Outcome<Self, String> {
        let mut string = String::new();
        if let Err(e) = data.open().read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        Success(BodyTest{ data: string })
    }
}




#[derive(Debug)]
pub struct Body {
   pub data: String
}

impl Body {
    pub fn get_data(&self) -> String {
        self.data.clone()
    }
}

impl FromDataSimple for Body {
    type Error = String;

    fn from_data(_: &Request, data: Data) -> data::Outcome<Self, String> {
        let mut string = String::new();
        if let Err(e) = data.open().read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        Success(Body{ data: string })
    }
}

// pub trait ChannelAccess {
//     fn get_channel_data(secret: &str, token: &str) -> Result<LineBot, env::VarError>;
// }

// impl ChannelAccess for LineBot {
//     fn get_channel_data(secret: &str, token: &str) -> Result<LineBot, env::VarError> {
//         let secret = env::var(secret)?;
//         let token = env::var(token)?;
//         Ok(LineBot::new(&secret, &token))
//     }
// }

pub struct Holiday {
    pub name: &'static str,
    pub day: &'static str,
}

impl Holiday {
    pub fn new(name: &'static str, day: &'static str) -> Self {
        Self {
            name,
            day
        }
    }
}