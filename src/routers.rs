pub mod auth;
pub mod auth_schema;
pub mod check_health;

use crate::utils::api_doc::ApiDoc;

use axum::http::Method;
use axum::{routing::get_service, Router};
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

pub fn create_app() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    let routes_static = Router::new().nest_service("/static", get_service(ServeDir::new("static")));
    Router::new()
        .merge(Scalar::with_url("/api/v1/api-reference", ApiDoc::openapi()))
        .merge(check_health::routes())
        .nest("/api/v1/auth", auth::routes())
        .fallback_service(routes_static)
        .layer(cors)
}
