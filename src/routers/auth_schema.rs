use crate::config::get_config;

use serde::{Deserialize, Serialize};
use std::result::Result as R;
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterIn {
    #[validate(length(min = 2, max = 64, message = "name must between 2-64 characters long"))]
    pub name: String,
    #[validate(email(message = "Invalid email"), custom(function = validate_email_domain, message = "invalid email domain"))]
    pub email: String,
    #[validate(length(min = 8, max = 32, message = "Must be 8 - 32 characters long"), custom(function = validate_strong_password))]
    pub password: String,
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

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginIn {
    #[validate(email(message = "Invalid email"), custom(function = validate_email_domain, message = "invalid email domain"))]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginOut {
    pub auth_token: String,
}
