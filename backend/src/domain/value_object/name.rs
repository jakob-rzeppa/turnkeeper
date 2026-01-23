use crate::domain::value_object::stat::key::Key;
use crate::error::DomainError;

pub struct Name {
    value: String,
}

impl Name {
    pub fn try_new(name: String) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::InvalidParameter("name cannot be empty".into()));
        }

        Ok(Self { value: name })
    }
}