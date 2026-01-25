use uuid::Uuid;
use crate::domain::entity::stat::Stat;
use crate::domain::entity::user::User;
use crate::domain::error::Error;
use crate::domain::value_object::identity::Identity;

/// The representation of a player
///
/// Use the `Player::builder()` for instantiating the Player.
pub struct Player {
    id: Identity,
    user: User,

    stats: Vec<Stat>
}

impl Player {
    pub fn new(id: Identity, user: User) -> Self {
        Self {
            id,
            user,
            stats: Vec::new()
        }
    }

    fn try_add_stat(&mut self, stat: Stat) -> Result<(), Error> {
        if self.stats.contains(&stat) {
            return Err(Error::InvalidState {
                msg: "stat for player already exists".to_string(),
            })
        }

        self.stats.push(stat);
        Ok(())
    }

    pub fn try_add_string_stat(&mut self, id: Identity, key: String, value: String) -> Result<(), Error> {
        let stat = Stat::try_new_string_stat(id, key, value).map_err(|e| e.prefix("player builder".to_string()))?;
        self.try_add_stat(stat)?;
        Ok(())
    }

    pub fn try_add_number_stat(&mut self, id: Identity, key: String, value: i64) -> Result<(), Error> {
        let stat = Stat::try_new_number_stat(id, key, value).map_err(|e| e.prefix("player builder".to_string()))?;
        self.try_add_stat(stat)?;
        Ok(())
    }

    pub fn try_add_bool_stat(&mut self, id: Identity, key: String, value: bool) -> Result<(), Error> {
        let stat = Stat::try_new_bool_stat(id, key, value).map_err(|e| e.prefix("player builder".to_string()))?;
        self.try_add_stat(stat)?;
        Ok(())
    }
}
