use crate::domain::error::Error;

/// # Invalid States
///
/// - key is empty
#[derive(Debug, Clone, PartialEq)]
pub struct StatKey {
    key: String,
}

impl StatKey {
    pub fn try_new(key: String) -> Result<Self, Error> {
        if key.is_empty() {
            return Err(Error::InvalidState { msg: "key cannot be empty".into() });
        }

        Ok(Self { key })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        let res = StatKey::try_new("key".to_string());

        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.key, "key".to_string());
    }

    #[test]
    fn test_new_empty_key() {
        let res = StatKey::try_new("".to_string());

        assert!(res.is_err());
        let res = res.unwrap_err();
        assert_eq!(res, Error::InvalidState { msg: "key cannot be empty".into() });
    }
}