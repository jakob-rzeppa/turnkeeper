use axum::Json;
use serde_json::Value;
use crate::error::HttpError;

pub async fn create_game() -> Result<Json<Value>, HttpError> {
    Err(HttpError::NotImplemented)
}