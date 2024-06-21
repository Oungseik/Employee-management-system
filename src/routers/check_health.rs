use axum::{response::IntoResponse, routing::get, Router};
use tracing::debug;

pub fn routes() -> Router {
    Router::new().route("/check-health", get(check_health))
}

async fn check_health<'a>() -> impl IntoResponse {
    debug!("GET /check-health");
    "server is up and running"
}
