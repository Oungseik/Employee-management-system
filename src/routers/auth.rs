use crate::error::{AppError, Result};
use axum::{routing::post, Extension, Json, Router};
use axum_valid::Valid;
use password_auth::generate_hash;
use serde::Deserialize;
use sqlx::{Pool, Sqlite};
use std::result::Result as R;
use validator::{Validate, ValidationError};

use crate::config::get_config;

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

#[derive(Debug, Deserialize, Validate)]
struct RegisterBody {
    #[validate(length(min = 2, max = 64, message = "name must between 2-64 characters long"))]
    name: String,
    #[validate(email(message = "Invalid email"), custom(function = validate_email_domain))]
    email: String,
    #[validate(length(min = 8, max = 32, message = "Must be 8 - 32 characters long"), custom(function = validate_strong_password))]
    password: String,
}

fn validate_email_domain(email: &str) -> R<(), ValidationError> {
    if !email.ends_with(&get_config().email_domain) {
        return Err(ValidationError::new("use valid email"));
    }
    Ok(())
}

fn validate_strong_password(password: &str) -> R<(), ValidationError> {
    if !(('a'..'z').any(|c| password.contains(c)) && ('A'..'Z').any(|c| password.contains(c))) {
        return Err(ValidationError::new(
            "password must contains at least one upper case and lower case character",
        ));
    }

    Ok(())
}
