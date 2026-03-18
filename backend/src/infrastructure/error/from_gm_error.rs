use crate::domain::gm::error::{GmError, GmErrorKind};
use crate::infrastructure::error::HttpError;

impl From<GmError> for HttpError {
    fn from(e: GmError) -> Self {
        match e.kind {
            GmErrorKind::InvalidCredentials => HttpError::Unauthorized(e.to_string()),
            GmErrorKind::Unauthorized => HttpError::Unauthorized(e.to_string()),
            GmErrorKind::JwtGenerationError => {
                eprintln!("{}", e);
                HttpError::InternalServerError
            },
        }
    }
}
