use crate::{
    config::get_config,
    error::{AppError, Result},
    modals::employee::Employee,
    utils::jwt::decode_auth_token,
};

use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use axum_extra::{
    extract::{cookie::Key, SignedCookieJar},
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
};
use jsonwebtoken::errors::ErrorKind;
use sqlx::{Pool, Sqlite};

pub async fn mw_authorization(mut req: Request, next: Next) -> Result<Response<Body>> {
    let signed_key = Key::from(get_config().cookies_signed_key.as_bytes());
    let jar = SignedCookieJar::from_headers(req.headers(), signed_key);
    let extract_bearer = || {
        req.headers()
            .typed_get::<Authorization<Bearer>>()
            .map(|v| v.token().to_owned())
    };

    let token = jar
        .get("auth_token")
        .map(|c| c.to_string())
        .or_else(extract_bearer)
        .ok_or(AppError::Unauthorized("token does not exist".to_string()))?;

    let claim = decode_auth_token(&token).map_err(|e| match e.kind() {
        &ErrorKind::ExpiredSignature => AppError::Unauthorized("token expired".to_string()),
        &ErrorKind::InvalidToken => AppError::Unauthorized("invalid token".to_string()),
        _ => AppError::Unauthorized("invalid token".to_string()),
    })?;

    let pool = req
        .extensions()
        .get::<Pool<Sqlite>>()
        .ok_or(AppError::InternalServerError(
            "something went wrong".to_string(),
        ))?;

    let employee = sqlx::query_as!(
        Employee,
        "SELECT * FROM employee WHERE email = $1",
        claim.email
    )
    .fetch_one(pool)
    .await
    .unwrap();

    req.extensions_mut().insert(employee);
    Ok(next.run(req).await)
}
