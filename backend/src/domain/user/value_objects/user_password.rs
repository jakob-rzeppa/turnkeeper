use crate::domain::user::error::{UserError, UserErrorKind};

/// # Invalid States
///
/// - password has fewer characters than 4
#[derive(Debug, Clone, PartialEq)]
pub struct UserPassword {
    value: String,
}

impl UserPassword {
    pub fn try_new(value: String) -> Result<Self, UserError> {
        if value.len() < 4 {
            return Err(UserError::new(UserErrorKind::PasswordTooShort { required: 4, actual: value.len() }));
        }

        Ok(Self { value })
    }

    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_new_valid() {
        let res = UserPassword::try_new("password".to_string());

        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.value, "password");
    }

    #[test]
    fn test_try_new_too_short() {
        let res = UserPassword::try_new("pas".to_string());

        assert!(res.is_err());
        let res = res.unwrap_err();
        assert_eq!(res, UserError::new(UserErrorKind::PasswordTooShort { required: 4, actual: 3 }));
    }
}