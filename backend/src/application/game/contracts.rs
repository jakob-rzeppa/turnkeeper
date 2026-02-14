use axum::response::sse::Event;
use uuid::Uuid;
use crate::domain::game::error::GameError;
use crate::domain::game::events::GameEvent;
use crate::domain::game::projections::GameMetadata;

#[mockall::automock]
pub trait GameRepositoryContract {
    async fn create(&self, id: Uuid, name: String) -> Result<(), GameError>;
    async fn get_metadata_all_games(&self) -> Result<Vec<GameMetadata>, GameError>;
    async fn get_metadata_by_id(&self, id: Uuid) -> Result<GameMetadata, GameError>;
    async fn log_event(&self, event: Event) -> Result<(), GameError>;
    async fn get_game_history(&self, id: Uuid) -> Result<Vec<GameEvent>, GameError>;
    async fn delete(&self, game_id: Uuid) -> Result<(), GameError>;
}