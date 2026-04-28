use async_trait::async_trait;

use crate::{
    application::common::error::DatabaseError,
    domain::{
        common::identifier::Identifier,
        game::{entities::game::Game, projections::game_metadata::GameMetadataProjection},
    },
};

#[mockall::automock]
#[async_trait]
pub trait GameRepositoryContract: Send + Sync + 'static {
    async fn get_by_id(&self, id: &Identifier) -> Result<Option<Game>, DatabaseError>;

    async fn list_all(&self) -> Result<Vec<GameMetadataProjection>, DatabaseError>;

    async fn save(&self, game: &Game) -> Result<(), DatabaseError>;

    async fn delete(&self, id: &Identifier) -> Result<(), DatabaseError>;
}
