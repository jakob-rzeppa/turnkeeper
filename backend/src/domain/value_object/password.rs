use crate::domain::error::Error;

/// # Invalid States
///
/// - password has fewer characters than 4
#[derive(Debug, Clone, PartialEq)]
pub struct Password {
    value: String,
}

impl Password {
    pub fn try_new(value: String) -> Result<Self, Error> {
        if value.len() < 4 {
            return Err(Error::InvalidState { msg: "password value must contain at least four characters".to_string() });
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
        let res = Password::try_new("password".to_string());

        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.value, "password");
    }

    #[test]
    fn test_try_new_too_short() {
        let res = Password::try_new("pas".to_string());

        assert!(res.is_err());
        let res = res.unwrap_err();
        assert_eq!(res, Error::InvalidState { msg: "password value must contain at least four characters".into() });
    }
}