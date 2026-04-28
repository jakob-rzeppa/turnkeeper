use crate::application::common::error::DatabaseError;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum GameApplicationError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),
    #[error("Game not found")]
    GameNotFound,
    #[error("Game has active instances and cannot be deleted")]
    GameHasInstances,
}
