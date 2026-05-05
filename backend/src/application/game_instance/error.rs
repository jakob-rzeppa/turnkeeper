use crate::{
    application::common::{error::DatabaseError, parser::error::ParsingError},
    domain::common::identifier::Id,
};

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum GameInstanceApplicationError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),
    #[error("Game instance with id {0} not found")]
    NotFound(Id),
    #[error("Game with id {0} for creating game instance not found")]
    GameNotFound(Id),
    #[error("Error parsing game: {0}")]
    ParsingError(#[from] ParsingError),
}
