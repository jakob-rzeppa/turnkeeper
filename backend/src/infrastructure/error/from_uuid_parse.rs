use crate::{domain::common::identifier::UuidParseError, infrastructure::error::HttpError};

impl From<UuidParseError> for HttpError {
    fn from(e: UuidParseError) -> Self {
        HttpError::BadRequest(format!("Invalid UUID: {}", e))
    }
}
