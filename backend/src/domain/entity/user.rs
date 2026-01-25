use uuid::Uuid;
use crate::domain::error::Error;
use crate::domain::value_object::identity::Identity;
use crate::domain::value_object::name::Name;
use crate::domain::value_object::password::Password;

/// The representation of a user
pub struct User {
    id: Identity,
    name: Name,
    // the password is stored in plain text,
    // so the gm can look up a password if a user forgot it
    password: Password,
}

impl User {
    pub fn try_new(id: Identity, name: String, password: String) -> Result<Self, Error> {
        let name = Name::try_new(name).map_err(|e| e.prefix("new user".to_string()))?;
        let password = Password::try_new(password).map_err(|e| e.prefix("new user".to_string()))?;
        
        Ok(Self { id, name, password })
    }
}
