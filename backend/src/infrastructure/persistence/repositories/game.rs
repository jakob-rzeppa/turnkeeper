use sqlx::SqlitePool;

use crate::{
    application::{common::error::DatabaseError, game::contracts::GameRepositoryContract},
    domain::{
        common::identifier::Identifier,
        game::{entities::game::Game, projections::game_metadata::GameMetadataProjection},
    },
};

pub struct SqliteGameRepository {
    db: SqlitePool,
}

impl SqliteGameRepository {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

impl GameRepositoryContract for SqliteGameRepository {
    async fn get_by_id(&self, id: Identifier) -> Result<Option<Game>, DatabaseError> {
        unimplemented!()
    }

    async fn save(&self, game: &Game) -> Result<(), DatabaseError> {
        unimplemented!()
    }

    async fn delete(&self, id: Identifier) -> Result<(), DatabaseError> {
        unimplemented!()
    }

    async fn list_all(&self) -> Result<Vec<GameMetadataProjection>, DatabaseError> {
        unimplemented!()
    }
}
