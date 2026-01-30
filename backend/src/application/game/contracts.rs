use uuid::Uuid;
use crate::domain::error::Error;
use crate::domain::game::entities::game::Game;

#[mockall::automock]
pub trait GameRepositoryContract {
    async fn save(&self, game: Game) -> Result<(), Error>;
    async fn delete(&self, game_id: Uuid) -> Result<(), Error>;
}