use crate::config::get_config;
use crate::error::{AppError, Result};
use crate::modals::employee::Employee;
use crate::routers::auth_schema::{LoginIn, LoginOut, RegisterIn};
use crate::utils::jwt::encode_auth_token;

use axum::http::StatusCode;
use axum::{response::IntoResponse, routing::post, Extension, Json, Router};
use axum_valid::Valid;
use chrono::Duration;
use password_auth::{generate_hash, verify_password};
use sqlx::{Pool, Sqlite};

pub fn routes() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

#[utoipa::path(
        post,
        path = "/api/v1/register",
        request_body = RegisterIn,
        responses(
            (status = 201, description = "create a user"),
            (status = 409, description = "email already taken", body = EmailTakenError),
            (status = 500, description = "inernal server error", body = InternalServerError),
        ),
        tag = "Authentication",
    )]
async fn register(
    Extension(pool): Extension<Pool<Sqlite>>,
    Valid(Json(body)): Valid<Json<RegisterIn>>,
) -> Result<impl IntoResponse> {
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
            AppError::EmailTaken("email already taken".to_string())
        }
        _ => AppError::InternalServerError("internal server error".to_string()),
    })?;

    Ok((StatusCode::CREATED, ()))
}

#[utoipa::path(
        post,
        path = "/api/v1/login",
        request_body = LoginIn,
        responses(
            (status = 200, description = "successfully login", body = LoginOut),
            (status = 401, description = "password does not match", body = UnauthorizedError),
            (status = 404, description = "email does not exist", body = NotFoundError),
            (status = 500, description = "inernal server error", body = InternalServerError),
        ),
        tag = "Authentication",
    )]
async fn login(
    Extension(pool): Extension<Pool<Sqlite>>,
    Valid(Json(body)): Valid<Json<LoginIn>>,
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
        _ => AppError::InternalServerError("internal server error".to_string()),
    })?;

    verify_password(body.password, &employee.password)
        .map_err(|_| AppError::Unauthorized("password does not match".to_string()))?;

    let auth_token = encode_auth_token(
        &employee.email,
        &employee.name,
        Duration::seconds(get_config().token_duration),
    )
    .unwrap();

    Ok(Json(LoginOut { auth_token }))
}
