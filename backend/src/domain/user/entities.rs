//! # User Entity
//!
//! Defines the User aggregate root.

use crate::domain::game::value_objects::id::Id;
use crate::domain::user::error::{UserError, UserErrorKind};
use crate::domain::user::value_objects::user_name::UserName;
use crate::domain::user::value_objects::user_password::UserPassword;

/// Represents a user who can participate in games.
///
/// # Fields
///
/// * `id` - Unique identifier (UUID)
/// * `name` - User's display name (validated via [`UserName`])
/// * `password` - User's password (validated via [`UserPassword`])
///
/// # Notes
///
/// The password is stored in plain text so GMs can look up passwords if users forget them.
/// This is acceptable for the use case of a private game system.
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    id: Id,
    name: UserName,
    // the password is stored in plain text,
    // so the gm can look up a password if a user forgot it
    password: UserPassword,
}

impl User {
    /// Creates a new user with validation.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the user
    /// * `name` - Display name (will be validated)
    /// * `password` - Password (will be validated)
    ///
    /// # Errors
    ///
    /// Returns [`UserError`] if:
    /// - Name is empty or contains invalid characters
    /// - Password doesn't meet requirements
    pub fn try_new(id: Id, name: String, password: String) -> Result<Self, UserError> {
        let name = UserName::try_new(name).map_err(|e| UserError::with_source(UserErrorKind::InvalidUser, Box::new(e)))?;
        let password = UserPassword::try_new(password).map_err(|e| UserError::with_source(UserErrorKind::InvalidUser, Box::new(e)))?;

        Ok(Self { id, name, password })
    }

    /// Returns the user's unique identifier.
    pub fn id(&self) -> &Id {
        &self.id
    }
    
    /// Returns the user's display name.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    
    /// Returns the user's password.
    ///
    /// # Security Note
    ///
    /// Passwords are stored in plain text for this application's specific use case.
    pub fn password(&self) -> &str {
        self.password.as_str()
    }

    /// Verifies if the provided password matches the user's password.
    ///
    /// # Arguments
    ///
    /// * `password` - The password to check
    ///
    /// # Errors
    ///
    /// Returns [`UserError`] with [`UserErrorKind::InvalidCredentials`] if:
    /// - The password format is invalid
    /// - The password doesn't match
    pub fn check_password(&self, password: String) -> Result<(), UserError> {
        let password = UserPassword::try_new(password).map_err(|e| UserError::with_source(UserErrorKind::InvalidCredentials, Box::new(e)))?;

        if password != self.password {
            return Err(UserError::new(UserErrorKind::InvalidCredentials))
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    mod check_password {
        use super::super::*;

        #[test]
        fn test_valid_password() {
            let user = User::try_new(
                Id::new(),
                "name".to_string(),
                "password".to_string(),
            ).unwrap();

            let res = user.check_password("password".to_string());

            assert!(res.is_ok());
        }

        #[test]
        fn test_empty_password() {
            let user = User::try_new(
                Id::new(),
                "name".to_string(),
                "password".to_string(),
            ).unwrap();

            let res = user.check_password("".to_string());

            assert!(res.is_err());
            let err = res.unwrap_err();
            assert_eq!(err, UserError::new(UserErrorKind::InvalidCredentials))
        }

        #[test]
        fn test_invalid_password() {
            let user = User::try_new(
                Id::new(),
                "name".to_string(),
                "password".to_string(),
            ).unwrap();

            let res = user.check_password("invalid".to_string());

            assert!(res.is_err());
            let err = res.unwrap_err();
            assert_eq!(err, UserError::new(UserErrorKind::InvalidCredentials))
        }
    }
}