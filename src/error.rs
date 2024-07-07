use axum::{http::StatusCode, response::IntoResponse};
use std::result::Result as R;
use thiserror::Error;

pub type Result<T> = R<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    EmailTaken(String),
    #[error("internal server error")]
    InternalServerError,
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Unauthorized(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::EmailTaken(msg) => (StatusCode::CONFLICT, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
        };
        body.into_response()
    }
}
