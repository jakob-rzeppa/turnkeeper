mod from_game;
mod from_game_instance;
mod from_user;
mod from_uuid_parse;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

/// HTTP error types that can be returned from API endpoints.
///
/// Each variant maps to a specific HTTP status code and error message format.
#[derive(Debug, PartialEq, Clone)]
pub enum HttpError {
    NotImplemented,
    NotFound(String),
    Conflict(String),
    InternalServerError,
    UnsupportedMediaType,
    BadRequest(String),
    Unauthorized(String),
    ValidationError(String),
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        match self {
            HttpError::NotImplemented => {
                let body = Json::from(json!({ "error": "not implemented" }));
                (StatusCode::NOT_IMPLEMENTED, body).into_response()
            }
            HttpError::NotFound(e) => {
                let body = Json::from(json!({ "error": e }));
                (StatusCode::NOT_FOUND, body).into_response()
            }
            HttpError::Conflict(e) => {
                let body = Json::from(json!({ "error": e }));
                (StatusCode::CONFLICT, body).into_response()
            }
            HttpError::InternalServerError => {
                let body = Json::from(json!({ "error": "Internal Server Error" }));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
            HttpError::UnsupportedMediaType => {
                let body = Json::from(json!({ "error": "Unsupported MediaType" }));
                (StatusCode::UNSUPPORTED_MEDIA_TYPE, body).into_response()
            }
            HttpError::BadRequest(e) => {
                let body = Json::from(json!({ "error": e }));
                (StatusCode::BAD_REQUEST, body).into_response()
            }
            HttpError::Unauthorized(e) => {
                let body = Json::from(json!({ "error": e }));
                (StatusCode::UNAUTHORIZED, body).into_response()
            }
            HttpError::ValidationError(e) => {
                let body = Json::from(json!({ "error": e }));
                (StatusCode::BAD_REQUEST, body).into_response()
            }
        }
    }
}
