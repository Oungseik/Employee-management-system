use axum::{routing::get, Router};
use tracing::debug;

pub fn routes() -> Router {
    Router::new().route("/check-health", get(check_health))
}

#[utoipa::path(get, path = "/check-health", responses(
    (status = 200, description = "server is up and running"),
))]
async fn check_health<'a>() -> &'static str {
    debug!("GET /check-health");
    "server is up and running"
}
