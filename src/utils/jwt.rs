use crate::config::get_config;

use chrono::{TimeDelta, Utc};
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims<'a> {
    email: &'a str,
    name: &'a str,
    exp: i64,
}

pub fn encode_auth_token(email: &str, name: &str, duration: TimeDelta) -> Result<String, Error> {
    encode(
        &Header::default(),
        &Claims {
            email,
            name,
            exp: (Utc::now() + duration).timestamp(),
        },
        &EncodingKey::from_secret(get_config().auth_secret.as_bytes()),
    )
}

#[derive(Clone, Deserialize)]
pub struct DecodePayload {
    pub email: String,
    pub name: String,
}

pub fn decode_auth_token(token: &str) -> Result<DecodePayload, Error> {
    decode::<DecodePayload>(
        token,
        &DecodingKey::from_secret(get_config().auth_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
