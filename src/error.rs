use std::error::Error;
use std::fmt;

use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    NotFound,
}

impl Error for AuthError {}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            AuthError::InvalidToken => write!(f, "client error: user provided invalid token"),
            AuthError::MissingCredentials => {
                write!(f, "client error: user does not provide the credentials")
            }
            AuthError::WrongCredentials => {
                write!(f, "client error: user provided wrong credentials")
            }
            AuthError::TokenCreation => {
                write!(f, "server error: error occured while creating the token")
            }
            AuthError::NotFound => {
                write!(f, "user not found")
            }
        }
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            AuthError::InvalidToken | AuthError::MissingCredentials => {
                (StatusCode::UNAUTHORIZED, "invalid token")
            }
            AuthError::WrongCredentials => (StatusCode::FORBIDDEN, "wrong credential"),
            AuthError::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error while create token",
            ),
            AuthError::NotFound => (StatusCode::NOT_FOUND, "not found"),
        };
        body.into_response()
    }
}
