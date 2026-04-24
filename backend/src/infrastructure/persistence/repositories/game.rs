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
    async fn list_all(&self) -> Result<Vec<GameMetadataProjection>, DatabaseError> {
        unimplemented!()
    }

    async fn get_by_id(&self, id: &Identifier) -> Result<Option<Game>, DatabaseError> {
        unimplemented!()
    }

    async fn save(&self, game: &Game) -> Result<(), DatabaseError> {
        unimplemented!()
    }

    async fn delete(&self, id: &Identifier) -> Result<(), DatabaseError> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::persistence::db::create_test_pool;

    use super::*;

    #[tokio::test]
    async fn test_save_and_get_by_id() {
        let db = create_test_pool().await;
        let repository = SqliteGameRepository::new(db);

        // Create a new game
        let game = Game::new_raw(
            Identifier::new(),
            "Test Game".to_string(),
            "A test game description".to_string(),
            "print('Hello, World!')".to_string(),
            crate::domain::common::date_time::DateTime::now(),
            crate::domain::common::date_time::DateTime::now(),
        );

        // Save the game
        repository.save(&game).await.unwrap();

        // Retrieve the game by ID
        let retrieved_game = repository.get_by_id(game.id()).await.unwrap();

        assert!(retrieved_game.is_some());
        let retrieved_game = retrieved_game.unwrap();
        assert_eq!(retrieved_game, game);
    }

    #[tokio::test]
    async fn test_save_and_list_all_ordered_by_updated_at() {
        let db = create_test_pool().await;
        let repository = SqliteGameRepository::new(db);

        // Create a new game
        let game1 = Game::new_raw(
            Identifier::new(),
            "Test Game 1".to_string(),
            "A test game description".to_string(),
            "print('Hello, World!')".to_string(),
            crate::domain::common::date_time::DateTime::now(),
            crate::domain::common::date_time::DateTime::now(),
        );
        let game2 = Game::new_raw(
            Identifier::new(),
            "Test Game 2".to_string(),
            "A test game description".to_string(),
            "print('Hello, World!')".to_string(),
            crate::domain::common::date_time::DateTime::now(),
            crate::domain::common::date_time::DateTime::now(),
        );

        // Save the games
        repository.save(&game1).await.unwrap();
        repository.save(&game2).await.unwrap();

        // List all games
        let games_metadata = repository.list_all().await.unwrap();

        assert_eq!(games_metadata.len(), 2);
        assert_eq!(&games_metadata[1].updated_at, game1.updated_at());
        assert_eq!(&games_metadata[0].updated_at, game2.updated_at()); // game2 should be first because it updated last

        assert_eq!(&games_metadata[1].id, game1.id());
        assert_eq!(&games_metadata[0].id, game2.id());
        assert_eq!(games_metadata[1].name, game1.name());
        assert_eq!(games_metadata[0].name, game2.name());
        assert_eq!(games_metadata[1].description, game1.description());
        assert_eq!(games_metadata[0].description, game2.description());
        assert_eq!(&games_metadata[1].created_at, game1.created_at());
        assert_eq!(&games_metadata[0].created_at, game2.created_at());
    }

    #[tokio::test]
    async fn test_save_and_delete() {
        let db = create_test_pool().await;
        let repository = SqliteGameRepository::new(db);

        // Create a new game
        let game = Game::new_raw(
            Identifier::new(),
            "Test Game".to_string(),
            "A test game description".to_string(),
            "print('Hello, World!')".to_string(),
            crate::domain::common::date_time::DateTime::now(),
            crate::domain::common::date_time::DateTime::now(),
        );

        // Save the game
        repository.save(&game).await.unwrap();

        // Delete the game
        repository.delete(game.id()).await.unwrap();

        // Try to retrieve the deleted game
        let retrieved_game = repository.get_by_id(game.id()).await.unwrap();
        assert!(retrieved_game.is_none());
    }
}
