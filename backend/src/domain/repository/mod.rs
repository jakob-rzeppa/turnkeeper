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