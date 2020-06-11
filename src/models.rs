
// // #[derive(FromDataSimple)]
// pub struct Body {
//     data: String,

// }

// // #[derive(FromRequest)]
// pub struct Signature {
//     pub key: String,
// }

// impl Body {
//     pub fn get_data(&self) -> String {
//         self.data.clone()
//     }
// }

// use std::io::Read;
// use rocket::data::{self, FromData, FromDataSimple, Transform, Transformed, DataStream};
// // use rocket::data::{self, FromData, FromDataSimple, Transform, Transformed, Outcome, DataStream};
// use rocket::{Request, Data, Outcome, Outcome::*};
// // use rocket::{Request, Data,};
// use rocket::http::{Status, ContentType};

// impl FromDataSimple for Body {
//     type Error = String;
    
//     fn from_data(_: &Request, data: Data) -> data::Outcome<Self, Self::Error>{
//         let mut buf = String::new();
//         data.open().read_to_string(&mut buf).unwrap();
//         // if let Ok(s) = data {
//             // s.open().read_to_string(&mut buf);
//         // }

//         Success(Body { data: buf })
//     }
// }

// 52 lines (41 sloc)  1.16 KB
  
use rocket::Outcome;
use rocket::Outcome::*;
use rocket::request::{self, Request, FromRequest};
use rocket::http::Status;
use rocket::Data;
use rocket::data::{self, FromDataSimple};
use line_messaging_api_rust;

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