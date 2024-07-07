use axum::{response::IntoResponse, routing::get, Extension, Router};

use crate::utils::jwt::DecodePayload;

pub fn routes() -> Router {
    Router::new().route("/", get(get_employee))
}

async fn get_employee(Extension(payload): Extension<DecodePayload>) -> impl IntoResponse {
    format!("{} - {}", payload.email, payload.name)
}
