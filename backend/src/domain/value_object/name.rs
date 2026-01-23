use crate::domain::value_object::stat::key::Key;
use crate::error::DomainError;

#[derive(Debug)]
pub struct Name {
    value: String,
}

impl Name {
    pub fn try_new(name: String) -> Result<Self, DomainError> {
        if name.is_empty() {
            return Err(DomainError::InvalidParameter { msg: "name cannot be empty".into() });
        }

        Ok(Self { value: name })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_new_valid() {
        let res = Name::try_new("test".to_string());

        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.value, "test");
    }

    #[test]
    fn test_try_new_empty() {
        let res = Name::try_new("".to_string());

        assert!(res.is_err());
        let res = res.unwrap_err();
        assert_eq!(res, DomainError::InvalidParameter { msg: "name cannot be empty".into() });
    }
}