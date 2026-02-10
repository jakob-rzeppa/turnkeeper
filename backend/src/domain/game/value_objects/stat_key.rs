use crate::domain::game::error::{GameError, GameErrorKind};

/// # Invalid States
///
/// - key is empty
#[derive(Debug, Clone, PartialEq)]
pub struct StatKey {
    key: String,
}

impl StatKey {
    pub fn try_new(key: String) -> Result<Self, GameError> {
        if key.is_empty() {
            return Err(GameError::new(GameErrorKind::EmptyStatKey));
        }

        Ok(Self { key })
    }

    pub fn as_str(&self) -> &str {
        &self.key
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
        assert_eq!(res, GameError::new(GameErrorKind::EmptyStatKey));
    }
}