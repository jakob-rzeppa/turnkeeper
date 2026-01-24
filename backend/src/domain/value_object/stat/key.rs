use crate::error::DomainError;

#[derive(Debug, Clone, PartialEq)]
pub struct Key {
    key: String,
}

impl Key {
    pub fn try_new(key: String) -> Result<Self, DomainError> {
        if key.is_empty() {
            return Err(DomainError::InvalidParameter { msg: "key cannot be empty".into() });
        }

        Ok(Self { key })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let res = Key::try_new("key".to_string());

        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.key, "key".to_string());
    }

    #[test]
    fn test_new_empty_key() {
        let res = Key::try_new("".to_string());

        assert!(res.is_err());
        let res = res.unwrap_err();
        assert_eq!(res, DomainError::InvalidParameter { msg: "key cannot be empty".into() });
    }
}