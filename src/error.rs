use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;
use std::result::Result as R;
use thiserror::Error;
use utoipa::ToSchema;

pub type Result<T> = R<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    EmailTaken(String),
    #[error("{0}")]
    InternalServerError(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Unauthorized(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::InternalServerError(message) => {
                InternalServerError { message }.into_response()
            }
            AppError::EmailTaken(message) => EmailTakenError { message }.into_response(),
            AppError::NotFound(message) => NotFoundError { message }.into_response(),
            AppError::Unauthorized(message) => UnauthorizedError { message }.into_response(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct InternalServerError {
    pub message: String,
}

impl IntoResponse for InternalServerError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({
        "message": self.message,
        }));
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EmailTakenError {
    pub message: String,
}

impl IntoResponse for EmailTakenError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({
        "message": self.message,
        }));
        (StatusCode::CONFLICT, body).into_response()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct NotFoundError {
    pub message: String,
}

impl IntoResponse for NotFoundError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({
        "message": self.message,
        }));
        (StatusCode::NOT_FOUND, body).into_response()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UnauthorizedError {
    pub message: String,
}

impl IntoResponse for UnauthorizedError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({
        "message": self.message,
        }));
        (StatusCode::UNAUTHORIZED, body).into_response()
    }
}
