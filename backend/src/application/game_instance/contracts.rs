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
pub trait GameInstanceRepositoryContract: Send + Sync + 'static {
    fn get_by_id(
        &self,
        id: Identifier,
    ) -> impl Future<Output = Result<Option<GameInstance>, DatabaseError>> + Send;

    fn list_by_game_id(
        &self,
        game_id: Identifier,
    ) -> impl Future<Output = Result<Vec<GameInstanceMetadataProjection>, DatabaseError>> + Send;

    fn save(
        &self,
        game_instance: &GameInstance,
    ) -> impl Future<Output = Result<(), DatabaseError>> + Send;

    fn delete(&self, id: Identifier) -> impl Future<Output = Result<(), DatabaseError>> + Send;
}
