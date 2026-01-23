use crate::error::DomainError;

pub struct Password {
    value: String,
}

impl Password {
    pub fn try_new(value: String) -> Result<Self, DomainError> {
        if value.len() < 4 {
            return Err(DomainError::InvalidParameter("password value must contain at least four characters".to_string()));
        }

        Ok(Self { value })
    }
}