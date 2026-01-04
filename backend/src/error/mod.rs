use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug)]
pub enum HttpError {
    NotImplemented,
    NotFound(String),
    Conflict(String),
    InternalServerError
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            HttpError::NotImplemented => (
                StatusCode::NOT_IMPLEMENTED, "not implemented".to_string()
            ),
            HttpError::NotFound(e) => (
                StatusCode::NOT_FOUND, e
            ),
            HttpError::Conflict(e) => (
                StatusCode::CONFLICT, e
            ),
            HttpError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()
            ),
        };

        let body = Json::from(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

#[derive(Debug)]
pub enum RepositoryError {
    NotFound(String),
    Conflict(String),
    InvalidParameter(String),
    Database(String),
}

impl Into<HttpError> for RepositoryError {
    fn into(self) -> HttpError {
        match self {
            RepositoryError::NotFound(e) => HttpError::NotFound(e),
            RepositoryError::Conflict(e) => HttpError::Conflict(e),
            // A InvalidParameter error should not be happening: request validation should happen before the repository layer
            RepositoryError::InvalidParameter(e) => {
                eprintln!("Repository received a invalid parameter: {}", e);
                HttpError::InternalServerError
            },
            // A database error includes unexpected errors from the db and should not happen
            RepositoryError::Database(e) => {
                eprintln!("Repository received a database error: {}", e);
                HttpError::InternalServerError
            }
        }
    }
}