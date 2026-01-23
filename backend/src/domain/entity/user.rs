use uuid::Uuid;
use crate::domain::value_object::identifier::Identifier;
use crate::domain::value_object::name::Name;
use crate::domain::value_object::password::Password;
use crate::error::DomainError;

/// The representation of a user
pub struct User {
    id: Identifier,
    name: Name,
    // the password is stored in plain text,
    // so the gm can look up a password if a user forgot it
    password: Password,
}

impl User {
    pub fn try_new(id: Uuid, name: String, password: String) -> Result<Self, DomainError> {
        let id = Identifier::new(id);
        let name = Name::try_new(name)?;
        let password = Password::try_new(password)?;
        
        Ok(Self { id, name, password })
    }
}