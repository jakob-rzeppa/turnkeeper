use axum::Json;
use serde_json::Value;
use crate::error::HttpError;

/// POST /user/login
///
/// authenticates a user via username and password and returns a JSON WEB TOKEN
pub async fn login() -> Result<Json<Value>, HttpError> {
    Err(HttpError::NotImplemented)
}

/// POST /user/register
///
/// registers a new user via username and password
pub async fn register() -> Result<Json<Value>, HttpError> {
    Err(HttpError::NotImplemented)
}