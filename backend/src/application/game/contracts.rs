use uuid::Uuid;
use crate::domain::game::entities::game::Game;
use crate::domain::game::error::GameError;
use crate::domain::game::projections::GameMetadata;

#[mockall::automock]
pub trait GameRepositoryContract {
    async fn save(&self, game: &Game) -> Result<(), GameError>;
    async fn update(&self, game: &Game) -> Result<(), GameError>;
    async fn find_by_id(&self, game_id: Uuid) -> Result<Game, GameError>;
    async fn get_all_games_metadata(&self) -> Result<Vec<GameMetadata>, GameError>;
    async fn delete(&self, game_id: Uuid) -> Result<(), GameError>;
}