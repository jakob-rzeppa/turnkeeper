#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("Action not found: {0}")]
    ActionNotFound(String),
    #[error("Invalid action: {0}")]
    InvalidAction(String),
    #[error("Runtime error: {0}")]
    RuntimeError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum GameInstanceError {
    #[error("Action execution error: {0}")]
    ActionExecutionError(#[from] ExecutionError),
    #[error("Player not found: {0}")]
    PlayerNotFound(String),
    #[error("Invalid player order")]
    InvalidPlayerOrder,
}
