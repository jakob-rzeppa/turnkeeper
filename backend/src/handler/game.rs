use axum::Json;
use serde_json::Value;
use crate::error::HttpError;

/// GET /games
///
/// returns a list of all available games
pub async fn games_get_all() -> Result<Json<Value>, HttpError> {
    Err(HttpError::NotImplemented)
}

/// GET /games/:id
///
/// returns the game state of the game with given id
pub async fn games_get() -> Result<Json<Value>, HttpError> {
    Err(HttpError::NotImplemented)
}

/// POST /games
///
/// creates a game and returns the initial game state
pub async fn games_create() -> Result<Json<Value>, HttpError> {
    Err(HttpError::NotImplemented)
}

/// DELETE /games
///
/// deletes a game if no current connection to it
pub async fn games_delete() -> Result<Json<Value>, HttpError> {
    Err(HttpError::NotImplemented)
}