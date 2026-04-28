use crate::{application::game::error::GameApplicationError, infrastructure::error::HttpError};

impl From<GameApplicationError> for HttpError {
    fn from(e: GameApplicationError) -> Self {
        match e {
            GameApplicationError::DatabaseError(_) => {
                eprintln!("Database error: {:?}", e);
                HttpError::InternalServerError
            }
            GameApplicationError::GameNotFound => HttpError::NotFound("Game not found".into()),
            GameApplicationError::GameHasInstances => {
                HttpError::Conflict("Cannot delete game with existing instances".into())
            }
        }
    }
}
