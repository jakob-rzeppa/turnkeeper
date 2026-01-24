use std::fmt::Display;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::{json};
use thiserror::Error;

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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum JwtError {
    TimeError(String),
    EncodeError(String),
    DecodeError(String),
}

impl Into<HttpError> for JwtError {
    fn into(self) -> HttpError {
        match self {
            JwtError::TimeError(e) => {
                eprintln!("Error when getting the time for jwt: {}", e);
                HttpError::InternalServerError
            }
            JwtError::EncodeError(e) => {
                eprintln!("Error when encoding the JWT: {}", e);
                HttpError::InternalServerError
            }
            JwtError::DecodeError(e) => {
                HttpError::Unauthorized(e)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DomainError {
    InvalidParameter {
        msg: String,
    },
    AlreadyExists {
        msg: String,
    }
}

impl DomainError {
    /// Adds a prefix to the message
    ///
    /// If `prefix("test module")` is used on `DomainError::InvalidParameter { msg: "id can't be null" }`
    /// will return `DomainError::InvalidParameter { msg: "test module: id can't be null" }`.
    pub fn prefix(self, prefix: String) -> Self {
        match self {
            DomainError::InvalidParameter { msg } => {
                DomainError::InvalidParameter { msg: format!("{0}: {1}", prefix, msg) }
            }
            DomainError::AlreadyExists { msg } => {
                DomainError::AlreadyExists { msg: format!("{0}: {1}", prefix, msg) }
            }
            _ => self,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ApplicationError {
    InvalidParameter {
        msg: String,
    },
    AlreadyExists {
        msg: String,
    },
    NotImplemented,
}