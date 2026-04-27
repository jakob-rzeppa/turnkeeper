#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum DatabaseError {
    #[error("Serialization Error: {0}")]
    SerializationError(String),
    #[error("Deserialization Error: {0}")]
    DeserializationError(String),
    #[error("{0}")]
    Unknown(String),
}
