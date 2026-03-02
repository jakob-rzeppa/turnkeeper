use uuid::Uuid;
use crate::domain::user::entities::User;
use crate::domain::game::entities::stat::Stat;
use crate::domain::game::error::{GameError, GameErrorKind};

/// A player within a game.
///
/// Created via [`Player::new`]. A player may optionally be linked to a [`User`]
/// and can hold any number of [`Stat`] entries.
///
/// # Invariants
///
/// - No two stats have the same key
#[derive(Debug, PartialEq, Clone)]
pub struct Player {
    id: Uuid,
    user_id: Option<Uuid>,

    stats: Vec<Stat>
}

impl Player {
    /// Creates a new anonymous player with no linked user and no stats.
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            user_id: None,
            stats: Vec::new()
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn user_id(&self) -> Option<Uuid> {
        self.user_id
    }

    pub fn stats(&self) -> &[Stat] {
        &self.stats
    }

    pub fn attach_user(&mut self, user_id: Uuid) {
        self.user_id = Some(user_id);
    }

    pub fn detach_user(&mut self) {
        self.user_id = None;
    }

    pub fn try_add_stat(&mut self, stat: Stat) -> Result<(), GameError> {
        if self.stats.iter().any(|s| s.key() == stat.key()) {
            return Err(GameError::new(GameErrorKind::DuplicateStatKey));
        }

        self.stats.push(stat);
        Ok(())
    }

    pub fn try_add_string_stat(&mut self, id: Uuid, key: String, value: String) -> Result<(), GameError> {
        let stat = Stat::try_new_string_stat(id, key, value)?;
        self.try_add_stat(stat)?;
        Ok(())
    }

    pub fn try_add_number_stat(&mut self, id: Uuid, key: String, value: f64) -> Result<(), GameError> {
        let stat = Stat::try_new_number_stat(id, key, value)?;
        self.try_add_stat(stat)?;
        Ok(())
    }

    pub fn try_add_bool_stat(&mut self, id: Uuid, key: String, value: bool) -> Result<(), GameError> {
        let stat = Stat::try_new_bool_stat(id, key, value)?;
        self.try_add_stat(stat)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_user() -> User {
        User::try_new(Uuid::new_v4(), "test-user".to_string(), "test-password".to_string()).unwrap()
    }

    #[test]
    fn test_add_duplicate_stat_fails() {
        let mut player = Player::new(Uuid::new_v4());
        let stat_id = Uuid::new_v4();
        let key = "score".to_string();
        let value = 42.0;

        // Add first stat
        assert!(player.try_add_number_stat(stat_id, key.clone(), value).is_ok());

        // Adding the same stat key again should fail
        let result = player.try_add_bool_stat(Uuid::new_v4(), key, true);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, GameError::new(GameErrorKind::DuplicateStatKey));
    }

    #[test]
    fn test_add_different_stats_succeeds() {
        let mut player = Player::new(Uuid::new_v4());
        assert!(player.try_add_number_stat(Uuid::new_v4(), "score".to_string(), 42.0).is_ok());
        assert!(player.try_add_string_stat(Uuid::new_v4(), "nickname".to_string(), "hero".to_string()).is_ok());
        assert!(player.try_add_bool_stat(Uuid::new_v4(), "active".to_string(), true).is_ok());
    }
}
