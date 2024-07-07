use crate::config::get_config;

use chrono::{TimeDelta, Utc};
use jsonwebtoken::{encode, errors::Error, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims<'a> {
    email: &'a str,
    exp: i64,
}

pub fn get_auth_token(email: &str, duration: TimeDelta) -> Result<String, Error> {
    encode(
        &Header::default(),
        &Claims {
            email,
            exp: (Utc::now() + duration).timestamp(),
        },
        &EncodingKey::from_secret(get_config().auth_secret.as_bytes()),
    )
}

pub fn get_refresh_toke(email: &str, duration: TimeDelta) -> Result<String, Error> {
    encode(
        &Header::default(),
        &Claims {
            email,
            exp: (Utc::now() + duration).timestamp(),
        },
        &EncodingKey::from_secret(get_config().refresh_secret.as_bytes()),
    )
}
