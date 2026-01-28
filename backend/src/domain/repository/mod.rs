use uuid::Uuid;
use crate::domain::auth::entities::User;
use crate::domain::entity::game::Game;
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
    async fn get_by_id(&self, id: Uuid) -> Result<User, Error>;

    async fn get_by_name(&self, name: String) -> Result<User, Error>;

    /// Save needs to check that the name is unique
    async fn save(&self, user: User) -> Result<(), Error>;
}
