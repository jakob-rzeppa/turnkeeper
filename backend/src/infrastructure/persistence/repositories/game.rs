use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::{
    application::{common::error::DatabaseError, game::contracts::GameRepositoryContract},
    domain::{
        common::identifier::Id,
        game::{entities::game::Game, projections::game_metadata::GameMetadataProjection},
    },
};

struct GameRow {
    id: String,
    name: String,
    description: String,
    source_code: String,
    created_at: String,
    updated_at: String,
}

impl From<&Game> for GameRow {
    fn from(game: &Game) -> Self {
        Self {
            id: game.id().to_string(),
            name: game.name().to_string(),
            description: game.description().to_string(),
            source_code: game.source_code().to_string(),
            created_at: game.created_at().to_string(),
            updated_at: game.updated_at().to_string(),
        }
    }
}

impl TryInto<Game> for GameRow {
    type Error = DatabaseError;

    fn try_into(self) -> Result<Game, Self::Error> {
        let id = Id::parse_str(&self.id)
            .map_err(|e| DatabaseError::Unknown(format!("Failed to parse id: {}", e)))?;
        let created_at = crate::domain::common::date_time::DateTime::parse_str(&self.created_at)
            .map_err(|e| DatabaseError::Unknown(format!("Failed to parse created_at: {}", e)))?;
        let updated_at = crate::domain::common::date_time::DateTime::parse_str(&self.updated_at)
            .map_err(|e| DatabaseError::Unknown(format!("Failed to parse updated_at: {}", e)))?;

        Ok(Game::new_raw(
            id,
            self.name,
            self.description,
            self.source_code,
            created_at,
            updated_at,
        ))
    }
}

pub struct SqliteGameRepository {
    db: SqlitePool,
}

impl SqliteGameRepository {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl GameRepositoryContract for SqliteGameRepository {
    async fn list_all(&self) -> Result<Vec<GameMetadataProjection>, DatabaseError> {
        let rows = sqlx::query!(
            r#"
            SELECT serialized FROM games
            "#
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| {
            DatabaseError::Unknown(format!("Database error while listing all Games: {}", e))
        })?;

        let mut metadata_list = rows
            .into_iter()
            .map(|row| {
                let game: Result<Game, DatabaseError> = serde_json::from_str(&row.serialized)
                    .map_err(|e| {
                        DatabaseError::DeserializationError(format!(
                            "Failed to deserialize Game for metadata projection: {}",
                            e
                        ))
                    });
                match game {
                    Ok(game) => Ok(game.get_metadata_projection()),
                    Err(e) => Err(e),
                }
            })
            .collect::<Result<Vec<GameMetadataProjection>, DatabaseError>>()?;

        metadata_list.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        Ok(metadata_list)
    }

    async fn get_by_id(&self, id: &Id) -> Result<Option<Game>, DatabaseError> {
        let id_str = id.to_string();

        let row = sqlx::query!(
            r#"
            SELECT serialized FROM games
            WHERE id = ?
            "#,
            id_str
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|e| {
            DatabaseError::Unknown(format!(
                "Database error while fetching Game with id {}: {}",
                id, e
            ))
        })?;

        if let Some(row) = row {
            let game: Game = serde_json::from_str(&row.serialized).map_err(|e| {
                DatabaseError::DeserializationError(format!(
                    "Failed to deserialize Game with id {}: {}",
                    id, e
                ))
            })?;
            Ok(Some(game))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, game: &Game) -> Result<(), DatabaseError> {
        let id_string = game.id().to_string();
        let serialized = serde_json::to_string(game).map_err(|e| {
            DatabaseError::SerializationError(format!(
                "Failed to serialize Game with id {}: {}",
                game.id(),
                e
            ))
        })?;

        sqlx::query!(
            r#"
            INSERT INTO games (id, serialized) VALUES (?, ?)
            ON CONFLICT(id) DO UPDATE SET serialized = excluded.serialized
            "#,
            id_string,
            serialized
        )
        .execute(&self.db)
        .await
        .map_err(|e| {
            DatabaseError::Unknown(format!(
                "Database error while saving Game with id {}: {}",
                game.id(),
                e
            ))
        })?;

        Ok(())
    }

    async fn delete(&self, id: &Id) -> Result<(), DatabaseError> {
        let id_string = id.to_string();

        sqlx::query!(
            r#"
            DELETE FROM games WHERE id = ?
            "#,
            id_string
        )
        .execute(&self.db)
        .await
        .map_err(|e| {
            DatabaseError::Unknown(format!(
                "Database error while deleting Game with id {}: {}",
                id, e
            ))
        })?;

        Ok(())
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
            Id::new(),
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
            Id::new(),
            "Test Game 1".to_string(),
            "A test game description".to_string(),
            "print('Hello, World!')".to_string(),
            crate::domain::common::date_time::DateTime::now(),
            crate::domain::common::date_time::DateTime::now(),
        );
        let game2 = Game::new_raw(
            Id::new(),
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
            Id::new(),
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
