use crate::domain::common::identifier::Identifier;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ExecutionError {
    #[error("Action not found: {0}")]
    ActionNotFound(String),
    #[error("Invalid action: {0}")]
    InvalidAction(String),
    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum GameInstanceError {
    #[error("Action execution error: {0}")]
    ActionExecutionError(#[from] ExecutionError),
    #[error("Player already exists: {0}")]
    PlayerAlreadyExists(String),
    #[error("Player not found: {0}")]
    PlayerNotFound(String),

    #[error("Stat with id {0} not found")]
    StatNotFound(Identifier),
    #[error("Player stat with id {0} not found")]
    PlayerStatNotFound(Identifier),
    #[error("Player with id {player_id} not found in stat {stat_name} (id: {stat_id})")]
    PlayerInStatNotFound {
        player_id: Identifier,
        stat_id: Identifier,
        stat_name: String,
    },

    #[error("Invalid player order")]
    InvalidPlayerOrder,
    #[error("User already attached to another player")]
    UserAlreadyAttachedToAnotherPlayer,
}
