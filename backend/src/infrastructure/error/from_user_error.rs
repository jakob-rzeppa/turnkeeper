use crate::domain::user::error::{UserError, UserErrorKind};
use crate::infrastructure::error::HttpError;

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
            UserErrorKind::InvalidToken => HttpError::Unauthorized(e.to_string()),
        }
    }
}
