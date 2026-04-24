use sqlx::SqlitePool;

use crate::{
    application::game_instance::contracts::GameInstanceRepositoryContract,
    domain::common::identifier::Identifier,
};

pub struct SqliteGameInstanceRepository {
    db: SqlitePool,
}

impl SqliteGameInstanceRepository {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

impl GameInstanceRepositoryContract for SqliteGameInstanceRepository {
    async fn get_by_id(
        &self,
        id: crate::domain::common::identifier::Identifier,
    ) -> Result<
        Option<crate::domain::game::entities::game_instance::GameInstance>,
        crate::application::common::error::DatabaseError,
    > {
        unimplemented!()
    }

    async  fn list_by_game_id(&self,game_id:crate::domain::common::identifier::Identifier,) -> Result<Vec<crate::domain::game::projections::game_instance_metadata::GameInstanceMetadataProjection>,crate::application::common::error::DatabaseError>{
        unimplemented!()
    }

    async fn save(
        &self,
        game_instance: &crate::domain::game::entities::game_instance::GameInstance,
    ) -> Result<(), crate::application::common::error::DatabaseError> {
        unimplemented!()
    }

    async fn delete(
        &self,
        game_id: Identifier,
        instance_id: Identifier,
    ) -> Result<(), crate::application::common::error::DatabaseError> {
        unimplemented!()
    }
}
