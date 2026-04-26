use crate::{
    application::{common::error::DatabaseError, game::root_parser::error::GameParsingError},
    domain::common::identifier::Identifier,
};

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum GameInstanceApplicationError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),
    #[error("Game instance with id {0} not found")]
    NotFound(Identifier),
    #[error("Game with id {0} for creating game instance not found")]
    GameNotFound(Identifier),
    #[error("Error parsing game: {0}")]
    GameParsingError(#[from] GameParsingError),
}
