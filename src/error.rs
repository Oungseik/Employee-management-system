use std::result::Result as R;
use thiserror::Error;

use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = R<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("internal server error")]
    InternalServerError,
    #[error("{0}")]
    EmailTaken(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            ),

            AppError::EmailTaken(msg) => (StatusCode::CONFLICT, msg),
        };
        body.into_response()
    }
}
