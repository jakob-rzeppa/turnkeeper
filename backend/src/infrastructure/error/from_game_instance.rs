use crate::{
    application::game_instance::error::GameInstanceApplicationError,
    infrastructure::error::HttpError,
};

impl From<GameInstanceApplicationError> for HttpError {
    fn from(e: GameInstanceApplicationError) -> Self {
        match e {
            GameInstanceApplicationError::DatabaseError(_) => {
                eprintln!("Database error: {:?}", e);
                HttpError::InternalServerError
            }
            GameInstanceApplicationError::GameNotFound(id) => {
                HttpError::NotFound(format!("Game with id {} not found", id))
            }
            GameInstanceApplicationError::NotFound(id) => {
                HttpError::NotFound(format!("Game instance with id {} not found", id))
            }
            GameInstanceApplicationError::ParsingError(err) => {
                HttpError::BadRequest("Error parsing game: ".to_string() + &err.to_string())
            }
        }
    }
}
