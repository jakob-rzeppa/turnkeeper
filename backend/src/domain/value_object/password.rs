use crate::error::DomainError;

#[derive(Debug)]
pub struct Password {
    value: String,
}

impl Password {
    pub fn try_new(value: String) -> Result<Self, DomainError> {
        if value.len() < 4 {
            return Err(DomainError::InvalidParameter { msg: "password value must contain at least four characters".to_string() });
        }

        Ok(Self { value })
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
        assert_eq!(res, DomainError::InvalidParameter { msg: "password value must contain at least four characters".into() });
    }
}