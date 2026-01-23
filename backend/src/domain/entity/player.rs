use uuid::Uuid;
use crate::domain::entity::stat::Stat;
use crate::domain::entity::user::User;
use crate::domain::value_object::identifier::Identifier;
use crate::error::DomainError;

/// The representation of a player
///
/// Use the `Player::builder()` for instantiating the Player.
pub struct Player {
    id: Identifier,
    user: User,

    stats: Vec<Stat>
}

impl Player {
    pub fn builder() -> PlayerBuilder {
        PlayerBuilder::default()
    }
}

#[derive(Default)]
pub struct PlayerBuilder {
    id: Option<Identifier>,
    user: Option<User>,

    stats: Vec<Stat>,
}

impl PlayerBuilder {
    pub fn add_id(mut self, id: Uuid) -> Self {
        self.id = Some(Identifier::new(id));
        self
    }

    pub fn try_add_user(mut self, user_id: Uuid, user_name: String, user_password: String) -> Result<Self, DomainError> {
        self.user = Some(User::try_new(user_id, user_name, user_password)?);
        Ok(self)
    }

    pub fn try_add_string_stat(mut self, id: Uuid, key: String, value: String) -> Result<Self, DomainError> {
        let stat = Stat::try_new_string_stat(id, key, value)?;
        self.stats.push(stat);
        Ok(self)
    }

    pub fn add_number_stat(mut self, id: Uuid, key: String, value: i64) -> Result<Self, DomainError> {
        let stat = Stat::new_number_stat(id, key, value)?;
        self.stats.push(stat);
        Ok(self)
    }

    pub fn add_bool_stat(mut self, id: Uuid, key: String, value: bool) -> Result<Self, DomainError> {
        let stat = Stat::new_bool_stat(id, key, value)?;
        self.stats.push(stat);
        Ok(self)
    }

    pub fn build(self) -> Result<Player, DomainError> {
        let id = match self.id {
            Some(id) => id,
            None => return Err(DomainError::InvalidParameter("id is required".to_string()))
        };
        let user = match self.user {
            Some(user) => user,
            None => return Err(DomainError::InvalidParameter("user is required".to_string()))
        };

        Ok(Player {
            id,
            user,
            stats: self.stats,
        })
    }
}