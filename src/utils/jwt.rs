use chrono::{Duration, Utc};
use jsonwebtoken::{encode, errors::Error, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims<'a> {
    email: &'a str,
    exp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtData<'a> {
    pub email: &'a str,
}

pub fn get_jwt(body: JwtData) -> Result<String, Error> {
    // FIXME - use appropriate key
    encode(
        &Header::default(),
        &Claims {
            email: body.email,
            exp: (Utc::now() + Duration::minutes(15)).timestamp(),
        },
        &EncodingKey::from_secret("mykey".as_bytes()),
    )
}
