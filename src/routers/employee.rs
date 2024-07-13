use axum::{response::IntoResponse, routing::get, Extension, Router};
use sqlx::{Pool, Sqlite};

use crate::utils::jwt::JwtPayload;

pub fn routes() -> Router {
    Router::new().route("/me", get(me))
}

#[utoipa::path(
        get,
        path = "/api/v1/employees/me",
        responses(
            (status = 200, description = "get your profile", body = String),
            (status = 500, description = "inernal server error", body = InternalServerError),
        ),
        tag = "Employee",
        security( ("Authorization" = []), ("Authorization" = []))
    )]
async fn me(
    Extension(conn): Extension<Pool<Sqlite>>,
    Extension(payload): Extension<JwtPayload>,
) -> impl IntoResponse {
    sqlx::query!("SELECT id FROM employee WHERE email = $1", payload.email)
        .fetch_one(&conn)
        .await
        .map(|employee| println!("{employee:?}"))
        .unwrap();

    "hello"
}

// #[utoipa::path(
//         post,
//         path = "/api/v1/employees",
//         responses(
//             (status = 200, description = "successfully login", body = String),
//             (status = 500, description = "inernal server error", body = InternalServerError),
//         ),
//         tag = "Employee",
//         security( ("Authorization" = []), ("Authorization" = []))
//     )]
// async fn get_employee(Extension(payload): Extension<DecodePayload>) -> impl IntoResponse {
//     format!("{} - {}", payload.email, payload.name)
// }
