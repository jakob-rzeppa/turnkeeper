use crate::error::DomainError;

pub struct Key {
    key: String,
}

impl Key {
    pub fn try_new(key: String) -> Result<Self, DomainError> {
        if key.is_empty() {
            return Err(DomainError::InvalidParameter("key cannot be empty".into()));
        }

        Ok(Self { key })
    }
}