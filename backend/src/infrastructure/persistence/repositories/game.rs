use sqlx::SqlitePool;
use crate::application::game::contracts::GameRepositoryContract;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::game::events::GameEvent;
use crate::domain::game::projections::game_metadata::GameMetadata;
use crate::domain::game::value_objects::id::Id;

pub struct SqliteGameRepository {
    db: SqlitePool
}

impl SqliteGameRepository {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

impl GameRepositoryContract for SqliteGameRepository {
    async fn create(&self, id: Id, name: String) -> Result<(), GameError> {
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
                if err_msg.contains("UNIQUE constraint failed") {
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
                let id = Id::parse_str(&row.id)
                    .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;
                Ok(GameMetadata {
                    id,
                    name: row.name,
                })
            })
            .collect::<Result<Vec<_>, GameError>>()?;

        Ok(games)
    }

    async fn get_metadata_by_id(&self, id: Id) -> Result<GameMetadata, GameError> {
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
                let id = Id::parse_str(&row.id)
                    .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;
                Ok(GameMetadata {
                    id,
                    name: row.name,
                })
            }
            None => Err(GameError::new(GameErrorKind::GameNotFound)),
        }
    }

    async fn log_event(&self, game_id: Id, event: GameEvent) -> Result<(), GameError> {
        let game_id_str = game_id.to_string();

        // Check if game exists first
        let game_exists = sqlx::query!(
            r#"SELECT id FROM games WHERE id = ?"#,
            game_id_str
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        if game_exists.is_none() {
            return Err(GameError::new(GameErrorKind::GameNotFound));
        }

        // Serialize event to JSON
        let event_json = serde_json::to_string(&event)
            .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        // Insert event into games_log
        sqlx::query!(
            r#"
            INSERT INTO games_log (game_id, event)
            VALUES (?, ?)
            "#,
            game_id_str,
            event_json
        )
        .execute(&self.db)
        .await
        .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        Ok(())
    }

    async fn get_game_history(&self, game_id: Id) -> Result<Vec<GameEvent>, GameError> {
        let game_id_str = game_id.to_string();

        // Check if game exists first
        let game_exists = sqlx::query!(
            r#"SELECT id FROM games WHERE id = ?"#,
            game_id_str
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        if game_exists.is_none() {
            return Err(GameError::new(GameErrorKind::GameNotFound));
        }

        // Fetch all events for this game
        let rows = sqlx::query!(
            r#"
            SELECT event
            FROM games_log
            WHERE game_id = ?
            ORDER BY timestamp ASC
            "#,
            game_id_str
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        // Deserialize events from JSON
        let events = rows
            .into_iter()
            .map(|row| {
                serde_json::from_str(&row.event)
                    .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))
            })
            .collect::<Result<Vec<GameEvent>, GameError>>()?;

        Ok(events)
    }

    async fn delete(&self, game_id: Id) -> Result<(), GameError> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::infrastructure::persistence::db::create_test_pool;
    use super::*;

    #[tokio::test]
    async fn test_create_and_get_game_metadata() {
        let db = create_test_pool().await;
        let repo = SqliteGameRepository::new(db);

        let game_id = Id::new();
        let game_name = "Test Game".to_string();

        let res = repo.create(game_id.clone(), game_name.clone()).await;
        assert!(res.is_ok());

        let metadata = repo.get_metadata_by_id(game_id.clone()).await;
        assert!(metadata.is_ok());
        let metadata = metadata.unwrap();
        assert_eq!(metadata.id, game_id);
        assert_eq!(metadata.name, game_name);
    }

    #[tokio::test]
    async fn test_create_game_twice() {
        let db = create_test_pool().await;
        let repo = SqliteGameRepository::new(db);

        let game_name = "Test Game".to_string();

        let res1 = repo.create(Id::new(), game_name.clone()).await;
        assert!(res1.is_ok());

        let res2 = repo.create(Id::new(), game_name.clone()).await;
        assert!(res2.is_err());
        let err = res2.err().unwrap();
        assert_eq!(err.kind, GameErrorKind::GameAlreadyExists);
    }

    #[tokio::test]
    async fn test_get_metadata_nonexistent_game() {
        let db = create_test_pool().await;
        let repo = SqliteGameRepository::new(db);

        let res = repo.get_metadata_by_id(Id::new()).await;
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert_eq!(err.kind, GameErrorKind::GameNotFound);
    }

    #[tokio::test]
    async fn test_get_metadata_all_games() {
        let db = create_test_pool().await;
        let repo = SqliteGameRepository::new(db);

        // Create some games
        let game1_id = Id::new();
        let game2_id = Id::new();
        repo.create(game1_id.clone(), "Game 1".to_string()).await.unwrap();
        repo.create(game2_id.clone(), "Game 2".to_string()).await.unwrap();

        let res = repo.get_metadata_all_games().await;
        assert!(res.is_ok());
        let games = res.unwrap();
        assert_eq!(games.len(), 2);
        assert!(games.iter().any(|g| g.id == game1_id && g.name == "Game 1"));
        assert!(games.iter().any(|g| g.id == game2_id && g.name == "Game 2"));
    }

    #[tokio::test]
    async fn test_get_metadata_all_games_empty() {
        let db = create_test_pool().await;
        let repo = SqliteGameRepository::new(db);

        let res = repo.get_metadata_all_games().await;
        assert!(res.is_ok());
        let games = res.unwrap();
        assert!(games.is_empty());
    }

    #[tokio::test]
    async fn test_log_events_and_get_game_history() {
        let db = create_test_pool().await;
        let repo = SqliteGameRepository::new(db);

        let game_id = Id::new();
        repo.create(game_id.clone(), "Event Test Game".to_string()).await.unwrap();

        let event1 = GameEvent::SetNotes("test notes".to_string());
        let res = repo.log_event(game_id.clone(), event1.clone()).await;
        assert!(res.is_ok());

        let event2 = GameEvent::AddPlayer {
            player_id: Id::new(),
        };
        let res = repo.log_event(game_id.clone(), event2.clone()).await;
        assert!(res.is_ok());

        let event3 = GameEvent::AddPlayer {
            player_id: Id::new(),
        };
        let res = repo.log_event(game_id.clone(), event3.clone()).await;
        assert!(res.is_ok());

        let event4 = GameEvent::AddStatToPlayer {
            player_id: Id::new(),
            stat_id: Id::new(),
            stat_key: "Strength".to_string(),
            stat_type: "number".to_string(),
            stat_value: "10".to_string(),
        };
        let res = repo.log_event(game_id.clone(), event4.clone()).await;
        assert!(res.is_ok());

        let history_res = repo.get_game_history(game_id.clone()).await;
        assert!(history_res.is_ok());
        let history = history_res.unwrap();
        assert_eq!(history.len(), 4);
        assert_eq!(history[0], event1);
        assert_eq!(history[1], event2);
        assert_eq!(history[2], event3);
        assert_eq!(history[3], event4);
    }

    #[tokio::test]
    async fn test_get_game_history_nonexistent_game() {
        let db = create_test_pool().await;
        let repo = SqliteGameRepository::new(db);

        let res = repo.get_game_history(Id::new()).await;
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert_eq!(err.kind, GameErrorKind::GameNotFound);
    }

    #[tokio::test]
    async fn test_get_game_history_empty() {
        let db = create_test_pool().await;
        let repo = SqliteGameRepository::new(db);

        let game_id = Id::new();
        repo.create(game_id.clone(), "Empty History Game".to_string()).await.unwrap();

        let res = repo.get_game_history(game_id.clone()).await;
        assert!(res.is_ok());
        let history = res.unwrap();
        assert!(history.is_empty());
    }

    #[tokio::test]
    async fn test_log_event_invalid_game() {
        let db = create_test_pool().await;
        let repo = SqliteGameRepository::new(db);

        let event = GameEvent::SetNotes("test notes".to_string());
        let res = repo.log_event(Id::new(), event.clone()).await;
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert_eq!(err.kind, GameErrorKind::GameNotFound);
    }
}