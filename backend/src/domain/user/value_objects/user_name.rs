use crate::domain::error::Error;

/// # Invalid States
///
/// - value is empty
#[derive(Debug, Clone, PartialEq)]
pub struct UserName {
    value: String,
}

impl UserName {
    pub fn try_new(name: String) -> Result<Self, Error> {
        if name.is_empty() {
            return Err(Error::InvalidState { msg: "name cannot be empty".into() });
        }

        Ok(Self { value: name })
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
        let res = UserName::try_new("test".to_string());

        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.value, "test");
    }

    #[test]
    fn test_try_new_empty() {
        let res = UserName::try_new("".to_string());

        assert!(res.is_err());
        let res = res.unwrap_err();
        assert_eq!(res, Error::InvalidState { msg: "name cannot be empty".into() });
    }
}