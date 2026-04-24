use crate::{
    application::common::error::DatabaseError,
    domain::{
        common::identifier::Identifier,
        game::{entities::game::Game, projections::game_metadata::GameMetadataProjection},
    },
};

#[mockall::automock]
pub trait GameRepositoryContract: Send + Sync + 'static {
    fn get_by_id(
        &self,
        id: Identifier,
    ) -> impl Future<Output = Result<Option<Game>, DatabaseError>> + Send;

    fn list_all(
        &self,
    ) -> impl Future<Output = Result<Vec<GameMetadataProjection>, DatabaseError>> + Send;

    fn save(&self, game: &Game) -> impl Future<Output = Result<(), DatabaseError>> + Send;

    fn delete(&self, id: Identifier) -> impl Future<Output = Result<(), DatabaseError>> + Send;
}
