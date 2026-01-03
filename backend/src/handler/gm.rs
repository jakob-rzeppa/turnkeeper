use axum::Json;
use serde_json::Value;
use crate::error::HttpError;

/// POST /gm/login
///
/// authenticates the gm via a secret set in the environment variables
/// and returns a JSON WEB TOKEN
pub async fn login() -> Result<Json<Value>, HttpError> {
    Err(HttpError::NotImplemented)
}