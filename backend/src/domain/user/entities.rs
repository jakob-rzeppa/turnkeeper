use uuid::Uuid;
use crate::domain::user::value_objects::user_name::UserName;
use crate::domain::user::value_objects::user_password::UserPassword;
use crate::domain::error::Error;

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
    pub fn try_new(id: Uuid, name: String, password: String) -> Result<Self, Error> {
        let name = UserName::try_new(name).map_err(|e| e.prefix("new user".to_string()))?;
        let password = UserPassword::try_new(password).map_err(|e| e.prefix("new user".to_string()))?;

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

    pub fn check_password(&self, password: String) -> Result<(), Error> {
        let password = UserPassword::try_new(password).map_err(|e| Error::InvalidCredentials {
            msg: "Wrong password".to_string()
        })?;

        if password != self.password {
            return Err(Error::InvalidCredentials {
                msg: "Wrong password".to_string(),
            })
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    mod check_password {
        use uuid::Uuid;
        use crate::domain::error::Error;
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
            assert_eq!(err, Error::InvalidCredentials {
                msg: "Wrong password".to_string()
            })
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
            assert_eq!(err, Error::InvalidCredentials {
                msg: "Wrong password".to_string()
            })
        }
    }
}