use sqlx::SqlitePool;
use crate::application::game::contracts::GameRepositoryContract;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::game::commands::GameCommand;
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

    async fn log_command(&self, game_id: Id, command: GameCommand) -> Result<(), GameError> {
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

        // Serialize command to JSON
        let command_json = serde_json::to_string(&command)
            .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        // Insert command into games_log
        sqlx::query!(
            r#"
            INSERT INTO games_log (game_id, command)
            VALUES (?, ?)
            "#,
            game_id_str,
            command_json
        )
        .execute(&self.db)
        .await
        .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        Ok(())
    }

    async fn get_game_history(&self, game_id: Id) -> Result<Vec<GameCommand>, GameError> {
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

        // Fetch all commands for this game
        let rows = sqlx::query!(
            r#"
            SELECT command
            FROM games_log
            WHERE game_id = ?
            ORDER BY timestamp ASC
            "#,
            game_id_str
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        // Deserialize commands from JSON
        let commands = rows
            .into_iter()
            .map(|row| {
                serde_json::from_str(&row.command)
                    .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))
            })
            .collect::<Result<Vec<GameCommand>, GameError>>()?;

        Ok(commands)
    }

    async fn delete(&self, _game_id: Id) -> Result<(), GameError> {
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
    async fn test_log_commands_and_get_game_history() {
        let db = create_test_pool().await;
        let repo = SqliteGameRepository::new(db);

        let game_id = Id::new();
        repo.create(game_id.clone(), "Command Test Game".to_string()).await.unwrap();

        let command1 = GameCommand::SetNotes("test notes".to_string());
        let res = repo.log_command(game_id.clone(), command1.clone()).await;
        assert!(res.is_ok());

        let command2 = GameCommand::AddPlayer {
            player_id: Id::new(),
        };
        let res = repo.log_command(game_id.clone(), command2.clone()).await;
        assert!(res.is_ok());

        let command3 = GameCommand::AddPlayer {
            player_id: Id::new(),
        };
        let res = repo.log_command(game_id.clone(), command3.clone()).await;
        assert!(res.is_ok());

        let command4 = GameCommand::AddStatToPlayer {
            player_id: Id::new(),
            stat_id: Id::new(),
            stat_key: "Strength".to_string(),
            stat_type: "number".to_string(),
            stat_value: "10".to_string(),
        };
        let res = repo.log_command(game_id.clone(), command4.clone()).await;
        assert!(res.is_ok());

        let history_res = repo.get_game_history(game_id.clone()).await;
        assert!(history_res.is_ok());
        let history = history_res.unwrap();
        assert_eq!(history.len(), 4);
        assert_eq!(history[0], command1);
        assert_eq!(history[1], command2);
        assert_eq!(history[2], command3);
        assert_eq!(history[3], command4);
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
    async fn test_log_command_invalid_game() {
        let db = create_test_pool().await;
        let repo = SqliteGameRepository::new(db);

        let command = GameCommand::SetNotes("test notes".to_string());
        let res = repo.log_command(Id::new(), command.clone()).await;
        assert!(res.is_err());
        let err = res.err().unwrap();
        assert_eq!(err.kind, GameErrorKind::GameNotFound);
    }
}