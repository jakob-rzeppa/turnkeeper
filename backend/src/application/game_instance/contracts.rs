use async_trait::async_trait;

use crate::{
    application::common::error::DatabaseError,
    domain::{
        common::identifier::Identifier,
        game::{
            entities::game_instance::GameInstance,
            projections::game_instance_metadata::GameInstanceMetadataProjection,
        },
    },
};

#[mockall::automock]
#[async_trait]
pub trait GameInstanceRepositoryContract: Send + Sync + 'static {
    async fn get_by_id(&self, id: Identifier) -> Result<Option<GameInstance>, DatabaseError>;

    async fn list_by_game_id(
        &self,
        game_id: Identifier,
    ) -> Result<Vec<GameInstanceMetadataProjection>, DatabaseError>;

    async fn game_has_instances(&self, game_id: Identifier) -> Result<bool, DatabaseError>;

    async fn save(&self, game_instance: &GameInstance) -> Result<(), DatabaseError>;

    async fn delete(
        &self,
        game_id: Identifier,
        instance_id: Identifier,
    ) -> Result<(), DatabaseError>;
}
