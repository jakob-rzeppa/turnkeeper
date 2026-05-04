#[derive(Debug, thiserror::Error)]
pub enum ActionInterpreterError {
    #[error("Action not found: {0}")]
    ActionNotFound(String),
}
