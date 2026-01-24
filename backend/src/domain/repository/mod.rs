use uuid::Uuid;
use crate::domain::entity::game::Game;
use crate::domain::entity::user::User;
use crate::error::DomainError;

pub trait GameRepositoryTrait {
    fn get_by_id(&self, game_id: Uuid) -> Result<Game, DomainError>;
    fn save(&self, game: Game) -> Result<(), DomainError>;
    fn update(&self, game: Game) -> Result<(), DomainError>;
    fn delete(&self, game_id: Uuid) -> Result<(), DomainError>;
}

pub trait UserRepositoryTrait {
    fn get_by_id(&self, user_id: Uuid) -> Result<Game, DomainError>;
    fn save(&self, user: User) -> Result<(), DomainError>;
    fn update(&self, game: Game) -> Result<(), DomainError>;
    fn delete(&self, user_id: Uuid) -> Result<(), DomainError>;
}
