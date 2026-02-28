use crate::domain::game::error::{GameError, GameErrorKind};
use crate::infrastructure::error::HttpError;

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
            GameErrorKind::UserAlreadyConnected => HttpError::Conflict(e.to_string()),
            GameErrorKind::GameSessionCreationFailed => HttpError::InternalServerError,
            GameErrorKind::GmAlreadyConnected => HttpError::Conflict(e.to_string()),
            GameErrorKind::InvalidPlayerOrder => HttpError::BadRequest(e.to_string()),
            GameErrorKind::NoPendingConnection => HttpError::Conflict(e.to_string()),
            GameErrorKind::InvalidConnectionToken => HttpError::Unauthorized(e.to_string()),
        }
    }
}
