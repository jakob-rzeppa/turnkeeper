use sqlx::SqlitePool;
use uuid::Uuid;
use crate::application::game::contracts::GameRepositoryContract;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::game::events::GameEvent;
use crate::domain::game::projections::GameMetadata;

pub struct SqliteGameRepository {
    db: SqlitePool
}

impl SqliteGameRepository {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

impl GameRepositoryContract for SqliteGameRepository {
    async fn create(&self, id: Uuid, name: String) -> Result<(), GameError> {
        let id_str = id.to_string();

        sqlx::query!(
            r#"
            INSERT INTO games (id, name)
            VALUES (?, ?)
            "#,
            id_str,
            name
        )
        .execute(&self.db)
        .await
        .map_err(|e| {
            // Check if this is a unique constraint error for name
            if let sqlx::Error::Database(db_err) = &e {
                let err_msg = db_err.message();
                if err_msg.contains("UNIQUE constraint failed") && err_msg.contains("name") {
                    return GameError::new(GameErrorKind::GameAlreadyExists);
                }
            }
            GameError::with_source(GameErrorKind::RepositoryError, Box::new(e))
        })?;

        Ok(())
    }

    async fn get_metadata_all_games(&self) -> Result<Vec<GameMetadata>, GameError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name
            FROM games
            "#
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        let games = rows
            .into_iter()
            .map(|row| {
                let id = Uuid::parse_str(&row.id)
                    .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;
                Ok(GameMetadata {
                    id,
                    name: row.name,
                })
            })
            .collect::<Result<Vec<_>, GameError>>()?;

        Ok(games)
    }

    async fn get_metadata_by_id(&self, id: Uuid) -> Result<GameMetadata, GameError> {
        let id_str = id.to_string();

        let row = sqlx::query!(
            r#"
            SELECT id, name
            FROM games
            WHERE id = ?
            "#,
            id_str
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        match row {
            Some(row) => {
                let id = Uuid::parse_str(&row.id)
                    .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;
                Ok(GameMetadata {
                    id,
                    name: row.name,
                })
            }
            None => Err(GameError::new(GameErrorKind::GameNotFound)),
        }
    }

    async fn log_event(&self, event: GameEvent) -> Result<(), GameError> {
        todo!()
    }

    async fn get_game_history(&self, game_id: Uuid) -> Result<Vec<GameEvent>, GameError> {
        todo!()
    }

    async fn delete(&self, game_id: Uuid) -> Result<(), GameError> {
        todo!()
    }
}