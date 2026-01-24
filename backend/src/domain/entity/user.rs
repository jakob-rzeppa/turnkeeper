use uuid::Uuid;
use crate::domain::value_object::name::Name;
use crate::domain::value_object::password::Password;
use crate::error::DomainError;

/// The representation of a user
pub struct User {
    id: Uuid,
    name: Name,
    // the password is stored in plain text,
    // so the gm can look up a password if a user forgot it
    password: Password,
}

impl User {
    pub fn try_new(id: Uuid, name: String, password: String) -> Result<Self, DomainError> {
        let name = Name::try_new(name).map_err(|e| e.prefix("new user".to_string()))?;
        let password = Password::try_new(password).map_err(|e| e.prefix("new user".to_string()))?;
        
        Ok(Self { id, name, password })
    }
}
