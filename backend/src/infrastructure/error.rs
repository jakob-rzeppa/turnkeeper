use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::{json};
use crate::domain::error::Error;
use crate::domain::user::error::{UserError, UserErrorKind};

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

impl From<Error> for HttpError {
    fn from(e: Error) -> Self {
        match e {
            Error::InvalidState { msg } => HttpError::BadRequest(msg),
            Error::InvalidCredentials { msg } => HttpError::Unauthorized(msg),
            Error::NotFound { msg } => HttpError::NotFound(msg),
            Error::DatabaseError { .. } => HttpError::InternalServerError,
            Error::UnexpectedError { .. } => HttpError::InternalServerError,
            Error::NotImplemented => HttpError::NotImplemented,
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
            UserErrorKind::JwtGenerationError(_) => {
                eprintln!("{}", e);
                HttpError::InternalServerError
            },
            UserErrorKind::DatabaseError(_) => {
                eprintln!("{}", e);
                HttpError::InternalServerError
            },
        }
    }
}