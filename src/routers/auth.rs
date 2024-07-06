use axum::{routing::post, Extension, Json, Router};
use axum_valid::Valid;
use chrono::Duration;
use password_auth::{generate_hash, verify_password};
use serde::Serialize;
use sqlx::{Pool, Sqlite};

use super::auth_schema::{LoginBody, RegisterBody};
use crate::error::{AppError, Result};
use crate::modals::employee::Employee;
use crate::utils::jwt::{get_auth_token, get_refresh_toke};

pub fn routes() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
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

async fn login(
    Extension(pool): Extension<Pool<Sqlite>>,
    Valid(Json(body)): Valid<Json<LoginBody>>,
) -> Result<Json<LoginOut>> {
    let employee = sqlx::query_as!(
        Employee,
        "SELECT * FROM employee WHERE email = ?",
        body.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => AppError::NotFound("user does not exist".to_string()),
        _ => AppError::InternalServerError,
    })?;

    verify_password(body.password, &employee.password)
        .map_err(|_| AppError::Unauthorized("password does not match".to_string()))?;

    let auth_token = get_auth_token(&body.email, Duration::minutes(15)).unwrap();
    let refresh_token = get_refresh_toke(&body.email, Duration::days(30)).unwrap();

    Ok(Json(LoginOut {
        auth_token,
        refresh_token,
    }))
}

#[derive(Debug, Serialize)]
struct LoginOut {
    auth_token: String,
    refresh_token: String,
}
