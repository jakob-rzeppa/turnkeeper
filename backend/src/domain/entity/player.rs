use uuid::Uuid;
use crate::domain::entity::stat::Stat;
use crate::domain::entity::user::User;
use crate::error::DomainError;

/// The representation of a player
///
/// Use the `Player::builder()` for instantiating the Player.
pub struct Player {
    id: Uuid,
    user: User,

    stats: Vec<Stat>
}

impl Player {
    pub fn new(id: Uuid, user: User) -> Self {
        Self {
            id,
            user,
            stats: Vec::new()
        }
    }

    fn try_add_stat(&mut self, stat: Stat) -> Result<(), DomainError> {
        if self.stats.contains(&stat) {
            return Err(DomainError::AlreadyExists {
                msg: "stat for player already exists".to_string(),
            })
        }

        self.stats.push(stat);
        Ok(())
    }

    pub fn try_add_string_stat(&mut self, id: Uuid, key: String, value: String) -> Result<(), DomainError> {
        let stat = Stat::try_new_string_stat(id, key, value).map_err(|e| e.prefix("player builder".to_string()))?;
        self.try_add_stat(stat)?;
        Ok(())
    }

    pub fn try_add_number_stat(&mut self, id: Uuid, key: String, value: i64) -> Result<(), DomainError> {
        let stat = Stat::try_new_number_stat(id, key, value).map_err(|e| e.prefix("player builder".to_string()))?;
        self.try_add_stat(stat)?;
        Ok(())
    }

    pub fn try_add_bool_stat(&mut self, id: Uuid, key: String, value: bool) -> Result<(), DomainError> {
        let stat = Stat::try_new_bool_stat(id, key, value).map_err(|e| e.prefix("player builder".to_string()))?;
        self.try_add_stat(stat)?;
        Ok(())
    }
}
