//! # HTTP Error Module
//!
//! Defines HTTP error types and their conversion to HTTP responses.
//!
//! ## Error Conversion
//!
//! Domain errors (UserError, GameError, GmError) are converted to appropriate
//! HTTP status codes and JSON error responses via the `From` trait implementations.

use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::{json};
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::gm::error::{GmError, GmErrorKind};
use crate::domain::user::error::{UserError, UserErrorKind};

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

impl From<UserError> for HttpError {
    fn from(e: UserError) -> Self {
        match e.kind {
            UserErrorKind::UserNotFound => HttpError::NotFound(e.to_string()),
            UserErrorKind::InvalidCredentials => HttpError::Unauthorized(e.to_string()),
            UserErrorKind::PasswordTooShort { .. } => HttpError::BadRequest(e.to_string()),
            UserErrorKind::EmptyName => HttpError::BadRequest(e.to_string()),
            UserErrorKind::InvalidUser => HttpError::BadRequest(e.to_string()),
            UserErrorKind::UserAlreadyExists => HttpError::Conflict(e.to_string()),
            UserErrorKind::JwtGenerationError => {
                eprintln!("{}", e);
                HttpError::InternalServerError
            },
            UserErrorKind::DatabaseError => {
                eprintln!("{}", e);
                HttpError::InternalServerError
            },
        }
    }
}

impl From<GmError> for HttpError {
    fn from(e: GmError) -> Self {
        match e.kind {
            GmErrorKind::InvalidCredentials => HttpError::Unauthorized(e.to_string()),
            GmErrorKind::JwtGenerationError => {
                eprintln!("{}", e);
                HttpError::InternalServerError
            },
        }
    }
}

impl From<GameError> for HttpError {
    fn from(e: GameError) -> Self {
        match e.kind {
            GameErrorKind::EmptyStatKey => HttpError::BadRequest(e.to_string()),
            GameErrorKind::InvalidStat => HttpError::BadRequest(e.to_string()),
            GameErrorKind::DuplicateStatKey => HttpError::Conflict(e.to_string()),
            GameErrorKind::GameAlreadyExists => HttpError::Conflict(e.to_string()),
            GameErrorKind::GameNotFound => HttpError::NotFound(e.to_string()),
            GameErrorKind::PlayerWithSameNameAlreadyExists => HttpError::Conflict(e.to_string()),
            GameErrorKind::RepositoryError => {
                eprintln!("{}", e);
                HttpError::InternalServerError
            },
            GameErrorKind::UserForPlayerNotFound => HttpError::NotFound(e.to_string()),
        }
    }
}