use uuid::Uuid;
use crate::domain::user::error::{UserError, UserErrorKind};
use crate::domain::user::value_objects::user_name::UserName;
use crate::domain::user::value_objects::user_password::UserPassword;

/// The representation of a user
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    id: Uuid,
    name: UserName,
    // the password is stored in plain text,
    // so the gm can look up a password if a user forgot it
    password: UserPassword,
}

impl User {
    pub fn try_new(id: Uuid, name: String, password: String) -> Result<Self, UserError> {
        let name = UserName::try_new(name).map_err(|e| UserError::with_source(UserErrorKind::InvalidUser, e))?;
        let password = UserPassword::try_new(password).map_err(|e| UserError::with_source(UserErrorKind::InvalidUser, e))?;

        Ok(Self { id, name, password })
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn password(&self) -> &str {
        self.password.as_str()
    }

    pub fn check_password(&self, password: String) -> Result<(), UserError> {
        let password = UserPassword::try_new(password).map_err(|e| UserError::with_source(UserErrorKind::InvalidCredentials, e))?;

        if password != self.password {
            return Err(UserError::new(UserErrorKind::InvalidCredentials))
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    mod check_password {
        use uuid::Uuid;
        use super::super::*;

        #[test]
        fn test_valid_password() {
            let user = User::try_new(
                Uuid::new_v4(),
                "name".to_string(),
                "password".to_string(),
            ).unwrap();

            let res = user.check_password("password".to_string());

            assert!(res.is_ok());
        }

        #[test]
        fn test_empty_password() {
            let user = User::try_new(
                Uuid::new_v4(),
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
                Uuid::new_v4(),
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