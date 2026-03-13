use crate::domain::game::error::{GameError, GameErrorKind};
use crate::infrastructure::error::HttpError;

impl From<GameError> for HttpError {
    fn from(e: GameError) -> Self {
        match e.kind {
            GameErrorKind::InvalidUuid => HttpError::BadRequest(e.to_string()),
            GameErrorKind::EmptyStatKey => HttpError::BadRequest(e.to_string()),
            GameErrorKind::InvalidStat(_) => HttpError::BadRequest(e.to_string()),
            GameErrorKind::StatNotFound => HttpError::NotFound(e.to_string()),
            GameErrorKind::DuplicateStatKey => HttpError::Conflict(e.to_string()),
            GameErrorKind::GameAlreadyExists => HttpError::Conflict(e.to_string()),
            GameErrorKind::GameNotFound => HttpError::NotFound(e.to_string()),
            GameErrorKind::PlayerNotFound => HttpError::NotFound(e.to_string()),
            GameErrorKind::PlayerAlreadyExists => HttpError::Conflict(e.to_string()),
            GameErrorKind::RepositoryError => {
                eprintln!("{}", e);
                HttpError::InternalServerError
            },
            GameErrorKind::UserAlreadyConnected => HttpError::Conflict(e.to_string()),
            GameErrorKind::UserNotInGame => HttpError::BadRequest(e.to_string()),
            GameErrorKind::GameSessionCreationFailed => {
                eprintln!("{}", e);
                HttpError::InternalServerError
            },
            GameErrorKind::GmAlreadyConnected => HttpError::Conflict(e.to_string()),
            GameErrorKind::InvalidPlayerOrder => HttpError::BadRequest(e.to_string()),
            GameErrorKind::NoPendingConnection => HttpError::Conflict(e.to_string()),
            GameErrorKind::InvalidConnectionToken => HttpError::Unauthorized(e.to_string()),
            GameErrorKind::UserAlreadyAttachedToAnotherPlayer => HttpError::Conflict(e.to_string()),
            GameErrorKind::GameSessionAlreadyExists => HttpError::Conflict(e.to_string()),
            GameErrorKind::GameHistoryInvalid => {
                eprintln!("{}", e);
                HttpError::InternalServerError
            },
            GameErrorKind::TradableAlreadyExists => HttpError::Conflict(e.to_string()),
            GameErrorKind::TradableNotFound => HttpError::NotFound(e.to_string()),
            GameErrorKind::TradablePlayerNotFound => HttpError::NotFound(e.to_string()),
            GameErrorKind::InsufficientTradableValue => HttpError::BadRequest(e.to_string()),
        }
    }
}
