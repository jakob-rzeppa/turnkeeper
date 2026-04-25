#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum DatabaseError {
    #[error("{0}")]
    Custom(String),
}
