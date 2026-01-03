use axum::response::IntoResponse;
use axum::http::StatusCode;
use serde::Serialize;
use axum::Json;
use crate::error::HttpError;
use crate::json_response;

json_response!(LoginResponse, {
    token: String,
});

/// POST /user/login
///
/// authenticates a user via username and password and returns a JSON WEB TOKEN
pub async fn login() -> Result<LoginResponse, HttpError> {
    Err(HttpError::NotImplemented)
}

json_response!(RegisterResponse, {
    token: String,
});

/// POST /user/register
///
/// registers a new user via username and password
pub async fn register() -> Result<RegisterResponse, HttpError> {
    Err(HttpError::NotImplemented)
}