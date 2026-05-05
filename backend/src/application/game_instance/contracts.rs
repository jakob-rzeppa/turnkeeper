use async_trait::async_trait;

use crate::{
    application::common::error::DatabaseError,
    domain::{
        common::identifier::Id,
        game::{
            entities::game_instance::GameInstance,
            projections::game_instance_metadata::GameInstanceMetadataProjection,
        },
    },
};

#[mockall::automock]
#[async_trait]
pub trait GameInstanceRepositoryContract: Send + Sync + 'static {
    async fn get_by_id(&self, id: Id) -> Result<Option<GameInstance>, DatabaseError>;

    async fn list_by_game_id(
        &self,
        game_id: Id,
    ) -> Result<Vec<GameInstanceMetadataProjection>, DatabaseError>;

    async fn game_has_instances(&self, game_id: Id) -> Result<bool, DatabaseError>;

    async fn save(&self, game_instance: &GameInstance) -> Result<(), DatabaseError>;

    async fn delete(
        &self,
        game_id: Id,
        instance_id: Id,
    ) -> Result<(), DatabaseError>;
}
