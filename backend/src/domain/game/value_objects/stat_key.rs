//! # Stat Key Value Object
//!
//! Provides a validated wrapper for stat keys.

use crate::domain::game::error::{GameError, GameErrorKind};

/// A validated key for a player statistic.
///
/// # Invariants
///
/// - Key must not be empty
#[derive(Debug, Clone, PartialEq)]
pub struct StatKey {
    key: String,
}

impl StatKey {
    /// Creates a new stat key with validation.
    ///
    /// # Arguments
    ///
    /// * `key` - The string to use as a stat key
    ///
    /// # Errors
    ///
    /// Returns [`GameError`] with [`GameErrorKind::EmptyStatKey`] if the key is empty.
    pub fn try_new(key: String) -> Result<Self, GameError> {
        if key.is_empty() {
            return Err(GameError::new(GameErrorKind::EmptyStatKey));
        }

        Ok(Self { key })
    }

    /// Returns the key as a string slice.
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