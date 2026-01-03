use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Serialize};
use crate::error::HttpError;
use crate::json_response;

json_response!(LoginResponse, {
    token: String,
});

/// POST /gm/login
///
/// authenticates the gm via a secret set in the environment variables
/// and returns a JSON WEB TOKEN
pub async fn login() -> Result<LoginResponse, HttpError> {
    Err(HttpError::NotImplemented)
}