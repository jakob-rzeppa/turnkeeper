use uuid::Uuid;
use crate::domain::entity::game::Game;
use crate::domain::entity::user::User;
use crate::domain::error::Error;

#[mockall::automock]
pub trait GameRepositoryTrait {
    fn get_by_id(&self, game_id: Uuid) -> Result<Game, Error>;
    fn save(&self, game: Game) -> Result<(), Error>;
    fn update(&self, game: Game) -> Result<(), Error>;
    fn delete(&self, game_id: Uuid) -> Result<(), Error>;
}

#[mockall::automock]
pub trait UserRepositoryTrait {
    fn get_by_id(&self, user_id: Uuid) -> Result<User, Error>;
    fn save(&self, user: User) -> Result<(), Error>;
    fn update(&self, user: User) -> Result<(), Error>;
    fn delete(&self, user_id: Uuid) -> Result<(), Error>;
}
