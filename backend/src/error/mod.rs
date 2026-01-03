use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug)]
pub enum HttpError {
    NotImplemented,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            HttpError::NotImplemented => (
                StatusCode::NOT_IMPLEMENTED, "not implemented"
            )
        };

        let body = Json::from(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}