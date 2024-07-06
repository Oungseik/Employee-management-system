use crate::error::{AppError, Result};
use axum::{routing::post, Extension, Json, Router};
use axum_valid::Valid;
use password_auth::generate_hash;
use sqlx::{Pool, Sqlite};

use super::auth_schema::RegisterBody;

pub fn routes() -> Router {
    Router::new().route("/register", post(register))
}

async fn register(
    Extension(pool): Extension<Pool<Sqlite>>,
    Valid(Json(body)): Valid<Json<RegisterBody>>,
) -> Result<()> {
    let hash = generate_hash(body.password);

    sqlx::query!(
        "INSERT INTO employee (name, email, password) VALUES ($1, $2, $3)",
        body.name,
        body.email,
        hash
    )
    .execute(&pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(err) if err.is_unique_violation() => {
            AppError::EmailTaken("email already exist".to_string())
        }
        _ => AppError::InternalServerError,
    })?;

    Ok(())
}
