use chrono::prelude::*;
use data::models::files::File;
use data::models::message_parser::{MessageParser, Segment};
use data::models::notifications::Notification;
use data::models::posts::Post;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};
use std::fs;

lazy_static! {
    static ref DECODING_KEY_RAW: Vec<u8> =
        fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../public.pem")).unwrap();
    static ref DECODING_KEY: DecodingKey<'static> =
        DecodingKey::from_rsa_pem(&DECODING_KEY_RAW).unwrap();
}

pub struct BearerToken<'a>(&'a str);

impl<'a, 'r> FromRequest<'a, 'r> for BearerToken<'a> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        let headers = request.headers();
        let header = headers.get("Authorization").last();
        match header {
            Some(header) => {
                let mut header = header.splitn(2, ' ');
                let auth_type = header.next().unwrap();
                if auth_type != "Bearer" {
                    return Outcome::Forward(());
                }

                let token = header.next().unwrap();
                Outcome::Success(BearerToken(token))
            }
            None => Outcome::Forward(()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_uuid: String,
    pub user_name: String,
    pub user_email: String,
    pub iat: u64,
    pub nbf: u64,
    pub exp: u64,
}

pub struct Authenticated(Claims);

impl Authenticated {
    pub fn get_uuid(&self) -> String {
        self.0.user_uuid.clone()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Authenticated {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, ()> {
        let BearerToken(token) = request.guard::<BearerToken>()?;

        let validation = Validation {
            algorithms: vec![Algorithm::RS256],
            validate_nbf: true,
            validate_exp: true,
            leeway: 60,
            aud: None,
            iss: None,
            sub: None,
        };

        match jsonwebtoken::decode::<Claims>(token, &*DECODING_KEY, &validation) {
            Ok(token_data) => Outcome::Success(Authenticated(token_data.claims)),
            Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct PostWithFiles {
    pub id: i32,
    pub name: String,
    pub tripcode: String,
    pub message_raw: String,
    pub message: Vec<Segment>,
    pub created_at: NaiveDateTime,
    pub files: Vec<File>,
    pub user_uuid: Option<String>,
}

impl PostWithFiles {
    pub fn new(post: Post, files: Vec<File>) -> PostWithFiles {
        PostWithFiles {
            id: post.id,
            name: post.name,
            tripcode: post.tripcode,
            message_raw: post.message.clone(),
            message: MessageParser::str_to_segments(&post.message),
            created_at: post.created_at,
            files,
            user_uuid: post.user_uuid,
        }
    }
}

#[derive(Serialize)]
pub struct NotificationWithPost {
    pub id: i32,
    pub user_uuid: String,
    pub read: bool,
    pub post: PostWithFiles,
}

impl NotificationWithPost {
    pub fn new(notification: Notification, post: PostWithFiles) -> NotificationWithPost {
        NotificationWithPost {
            id: notification.id,
            user_uuid: notification.user_uuid,
            read: notification.read,
            post,
        }
    }
}
