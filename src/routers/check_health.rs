use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/check-health", get(check_health))
}

async fn check_health<'a>() -> &'a str {
    "server is up and running"
}
