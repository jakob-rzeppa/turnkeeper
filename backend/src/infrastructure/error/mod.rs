//! # HTTP Error Module
//!
//! Defines HTTP error types and their conversion to HTTP responses.
//!
//! ## Error Conversion
//!
//! Domain errors (UserError, GameError, GmError) are converted to appropriate
//! HTTP status codes and JSON error responses via the `From` trait implementations.

mod from_game_error;
mod from_gm_error;
mod from_user_error;

use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

/// HTTP error types that can be returned from API endpoints.
///
/// Each variant maps to a specific HTTP status code and error message format.
#[derive(Debug, PartialEq)]
#[derive(Clone)]
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
            },
            HttpError::NotFound(e) => {
                let body = Json::from(json!({ "error": e }));
                (StatusCode::NOT_FOUND, body).into_response()
            },
            HttpError::Conflict(e) => {
                let body = Json::from(json!({ "error": e }));
                (StatusCode::CONFLICT, body).into_response()
            },
            HttpError::InternalServerError => {
                let body = Json::from(json!({ "error": "Internal Server Error" }));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            },
            HttpError::UnsupportedMediaType => {
                let body = Json::from(json!({ "error": "Unsupported MediaType" }));
                (StatusCode::UNSUPPORTED_MEDIA_TYPE, body).into_response()
            },
            HttpError::BadRequest(e) => {
                let body = Json::from(json!({ "error": e }));
                (StatusCode::BAD_REQUEST, body).into_response()
            },
            HttpError::Unauthorized(e) => {
                let body = Json::from(json!({ "error": e }));
                (StatusCode::UNAUTHORIZED, body).into_response()
            },
            HttpError::ValidationError(e) => {
                let body = Json::from(json!({ "error": e }));
                (StatusCode::BAD_REQUEST, body).into_response()
            }
        }
    }
}
