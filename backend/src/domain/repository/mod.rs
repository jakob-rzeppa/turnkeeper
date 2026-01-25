use uuid::Uuid;
use crate::domain::entity::game::Game;
use crate::domain::entity::user::User;
use crate::domain::error::Error;

#[mockall::automock]
pub trait GameRepositoryTrait {
    async fn get_by_id(&self, game_id: Uuid) -> Result<Game, Error>;
    async fn save(&self, game: Game) -> Result<(), Error>;
    async fn update(&self, game: Game) -> Result<(), Error>;
    async fn delete(&self, game_id: Uuid) -> Result<(), Error>;
}

#[mockall::automock]
pub trait UserRepositoryTrait {
    async fn get_by_id(&self, user_id: Uuid) -> Result<User, Error>;
    async fn save(&self, user: User) -> Result<(), Error>;
    async fn update(&self, user: User) -> Result<(), Error>;
    async fn delete(&self, user_id: Uuid) -> Result<(), Error>;
}
