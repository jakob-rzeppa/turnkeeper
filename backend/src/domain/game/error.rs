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

    #[error("Stat {0} not found")]
    StatNotFound(String),
    #[error("Player stat {0} not found")]
    PlayerStatNotFound(String),
    #[error("Player {player_name} not found in stat {stat_name}")]
    PlayerInStatNotFound {
        player_name: String,
        stat_name: String,
    },

    #[error("Invalid player order provided: {0:?}")]
    InvalidPlayerOrder(Vec<String>),
    #[error("User already attached to another player")]
    UserAlreadyAttachedToAnotherPlayer,
}
