use crate::{application::game::error::GameApplicationError, infrastructure::error::HttpError};

impl From<GameApplicationError> for HttpError {
    fn from(e: GameApplicationError) -> Self {
        match e {
            GameApplicationError::DatabaseError(_) => {
                eprintln!("Database error: {:?}", e);
                HttpError::InternalServerError
            }
            GameApplicationError::GameNotFound => HttpError::NotFound(e.to_string()),
            GameApplicationError::GameHasInstances => HttpError::Conflict(e.to_string()),
        }
    }
}
