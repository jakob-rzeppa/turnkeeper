//! SQLite implementation of the game repository.
//!
//! This module provides persistence for game entities using SQLite as the backend database.

use sqlx::SqlitePool;
use uuid::Uuid;
use crate::application::game::contracts::GameRepositoryContract;
use crate::domain::game::entities::game::Game;
use crate::domain::game::entities::player::Player;
use crate::domain::game::entities::stat::Stat;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::user::entities::User;

/// SQLite-based implementation of the game repository.
///
/// # Database Schema
///
/// The repository interacts with three main tables:
/// - `games`: Stores game metadata (id, name, round_number, current_player_index)
/// - `players`: Stores player data with foreign keys to games and users
/// - `stats`: Stores player statistics with various types (number, string, boolean)
pub struct SqliteGameRepository {
    db: SqlitePool
}

impl SqliteGameRepository {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }
}

impl GameRepositoryContract for SqliteGameRepository {
    /// Persists a new game entity to the database.
    ///
    /// This method creates a new game and all its related data:
    /// 1. Inserts the game record (fails if game with same name exists)
    /// 2. Inserts all players with their current position
    /// 3. Inserts all stats for each player
    ///
    /// # Transaction Behavior
    ///
    /// All operations are executed within a single database transaction. If any operation
    /// fails, the entire transaction is rolled back and no changes are persisted.
    ///
    /// # Arguments
    ///
    /// * `game` - A reference to the game entity to persist
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the game was successfully saved
    /// * `Err(GameError)` - With one of the following kinds:
    ///   - `GameErrorKind::GameAlreadyExists` - If a game with the same name already exists
    ///   - `GameErrorKind::UserForPlayerNotFound` - If a referenced user doesn't exist
    ///   - `GameErrorKind::RepositoryError` - If any other database operation fails
    ///
    /// # Important
    ///
    /// **Users must exist in the database before saving a game!** The players reference
    /// users via foreign keys. If a referenced user doesn't exist, the save operation will fail.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut game = Game::new(Uuid::new_v4(), \"My Game\".to_string());
    /// // ... add players to game
    /// repository.save(&game).await?;
    /// ```
    async fn save(&self, game: &Game) -> Result<(), GameError> {
        let mut tx = self.db.begin().await
            .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        let game_id = game.id().to_string();
        let game_name = game.name();
        let round_number = game.round_number() as i64;
        let current_player_index = game.current_player_index() as i64;

        // Insert game (will fail if name already exists)
        sqlx::query!(
            r#"
            INSERT INTO games (id, name, round_number, current_player_index)
            VALUES (?, ?, ?, ?)
            "#,
            game_id,
            game_name,
            round_number,
            current_player_index
        )
        .execute(&mut *tx)
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

        // Insert players and their stats
        for (position, player) in game.players().iter().enumerate() {
            let player_id = player.id().to_string();
            let user_id = player.user().id().to_string();
            let position = position as i64;

            sqlx::query!(
                r#"
                INSERT INTO players (id, game_id, user_id, position)
                VALUES (?, ?, ?, ?)
                "#,
                player_id,
                game_id,
                user_id,
                position
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                // Check if this is a foreign key constraint error for user_id
                if let sqlx::Error::Database(db_err) = &e {
                    let err_msg = db_err.message();
                    if err_msg.contains("FOREIGN KEY constraint failed") 
                        || (err_msg.contains("foreign key") && err_msg.contains("user")) {
                        return GameError::new(GameErrorKind::UserForPlayerNotFound);
                    }
                }
                GameError::with_source(GameErrorKind::RepositoryError, Box::new(e))
            })?;

            // Insert stats for this player
            for stat in player.stats() {
                let stat_id = stat.id().to_string();
                let stat_key = stat.key().as_str();
                let kind = stat.kind_str();
                let number_value = stat.as_number();
                let string_value = stat.as_string();
                let boolean_value = stat.as_boolean();

                sqlx::query!(
                    r#"
                    INSERT INTO stats (id, player_id, key, kind, number_value, string_value, boolean_value)
                    VALUES (?, ?, ?, ?, ?, ?, ?)
                    "#,
                    stat_id,
                    player_id,
                    stat_key,
                    kind,
                    number_value,
                    string_value,
                    boolean_value
                )
                .execute(&mut *tx)
                .await
                .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;
            }
        }

        tx.commit().await
            .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        Ok(())
    }

    /// Updates an existing game entity in the database.
    ///
    /// This method updates a game and all its related data:
    /// 1. Updates the game record (fails if game doesn't exist)
    /// 2. Deletes existing players (which cascades to stats)
    /// 3. Inserts all players with their current position
    /// 4. Inserts all stats for each player
    ///
    /// # Transaction Behavior
    ///
    /// All operations are executed within a single database transaction. If any operation
    /// fails, the entire transaction is rolled back and no changes are persisted.
    ///
    /// # Arguments
    ///
    /// * `game` - A reference to the game entity to update
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the game was successfully updated
    /// * `Err(GameError)` - With one of the following kinds:
    ///   - `GameErrorKind::GameNotFound` - If the game doesn't exist
    ///   - `GameErrorKind::UserForPlayerNotFound` - If a referenced user doesn't exist
    ///   - `GameErrorKind::RepositoryError` - If any other database operation fails
    ///
    /// # Important
    ///
    /// **Users must exist in the database before updating a game!** The players reference
    /// users via foreign keys. If a referenced user doesn't exist, the update operation will fail.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut game = repository.find_by_id(game_id).await?;
    /// // ... modify game
    /// repository.update(&game).await?;
    /// ```
    async fn update(&self, game: &Game) -> Result<(), GameError> {
        let mut tx = self.db.begin().await
            .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        let game_id = game.id().to_string();
        let game_name = game.name();
        let round_number = game.round_number() as i64;
        let current_player_index = game.current_player_index() as i64;

        // Update game (will fail if game doesn't exist)
        let result = sqlx::query!(
            r#"
            UPDATE games
            SET name = ?, round_number = ?, current_player_index = ?
            WHERE id = ?
            "#,
            game_name,
            round_number,
            current_player_index,
            game_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        if result.rows_affected() == 0 {
            return Err(GameError::new(GameErrorKind::GameNotFound));
        }

        // Delete existing players and their stats (cascade)
        sqlx::query!(
            r#"DELETE FROM players WHERE game_id = ?"#,
            game_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        // Insert players and their stats
        for (position, player) in game.players().iter().enumerate() {
            let player_id = player.id().to_string();
            let user_id = player.user().id().to_string();
            let position = position as i64;

            sqlx::query!(
                r#"
                INSERT INTO players (id, game_id, user_id, position)
                VALUES (?, ?, ?, ?)
                "#,
                player_id,
                game_id,
                user_id,
                position
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                // Check if this is a foreign key constraint error for user_id
                if let sqlx::Error::Database(db_err) = &e {
                    let err_msg = db_err.message();
                    if err_msg.contains("FOREIGN KEY constraint failed") 
                        || (err_msg.contains("foreign key") && err_msg.contains("user")) {
                        return GameError::new(GameErrorKind::UserForPlayerNotFound);
                    }
                }
                GameError::with_source(GameErrorKind::RepositoryError, Box::new(e))
            })?;

            // Insert stats for this player
            for stat in player.stats() {
                let stat_id = stat.id().to_string();
                let stat_key = stat.key().as_str();
                let kind = stat.kind_str();
                let number_value = stat.as_number();
                let string_value = stat.as_string();
                let boolean_value = stat.as_boolean();

                sqlx::query!(
                    r#"
                    INSERT INTO stats (id, player_id, key, kind, number_value, string_value, boolean_value)
                    VALUES (?, ?, ?, ?, ?, ?, ?)
                    "#,
                    stat_id,
                    player_id,
                    stat_key,
                    kind,
                    number_value,
                    string_value,
                    boolean_value
                )
                .execute(&mut *tx)
                .await
                .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;
            }
        }

        tx.commit().await
            .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        Ok(())
    }

    /// Retrieves a game entity by its unique identifier.
    ///
    /// This method performs multiple queries to reconstruct the complete game entity:
    /// 1. Fetches the game record
    /// 2. Fetches all players (with their associated users) ordered by position
    /// 3. For each player, fetches all their stats
    /// 4. Reconstructs the domain entities from the database records
    async fn find_by_id(&self, game_id: Uuid) -> Result<Game, GameError> {
        let game_id_str = game_id.to_string();

        // Query the game
        let game_row = sqlx::query!(
            r#"
            SELECT id, name, round_number, current_player_index
            FROM games
            WHERE id = ?
            "#,
            game_id_str
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?
        .ok_or_else(|| GameError::new(GameErrorKind::GameNotFound))?;

        // Query players with their users
        let player_rows = sqlx::query!(
            r#"
            SELECT 
                p.id as player_id,
                p.user_id,
                p.position,
                u.name as user_name,
                u.password as user_password
            FROM players p
            JOIN users u ON p.user_id = u.id
            WHERE p.game_id = ?
            ORDER BY p.position
            "#,
            game_id_str
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        // Build game
        let mut game = Game::new(game_id, game_row.name);

        // Reconstruct players
        for player_row in player_rows {
            let player_id = Uuid::parse_str(&player_row.player_id)
                .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;
            let user_id = Uuid::parse_str(&player_row.user_id)
                .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

            let user = User::try_new(
                user_id,
                player_row.user_name,
                player_row.user_password,
            ).map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

            let mut player = Player::new(player_id, user);

            // Query stats for this player
            let stat_rows = sqlx::query!(
                r#"
                SELECT id, key, kind, number_value, string_value, boolean_value
                FROM stats
                WHERE player_id = ?
                "#,
                player_row.player_id
            )
            .fetch_all(&self.db)
            .await
            .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

            // Reconstruct stats
            for stat_row in stat_rows {
                let stat_id = Uuid::parse_str(&stat_row.id)
                    .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

                let stat = match stat_row.kind.as_str() {
                    "number" => {
                        let value = stat_row.number_value
                            .ok_or_else(|| GameError::new(GameErrorKind::RepositoryError))?;
                        Stat::try_new_number_stat(stat_id, stat_row.key, value)?
                    }
                    "string" => {
                        let value = stat_row.string_value
                            .ok_or_else(|| GameError::new(GameErrorKind::RepositoryError))?;
                        Stat::try_new_string_stat(stat_id, stat_row.key, value)?
                    }
                    "boolean" => {
                        let value = stat_row.boolean_value
                            .ok_or_else(|| GameError::new(GameErrorKind::RepositoryError))?;
                        Stat::try_new_bool_stat(stat_id, stat_row.key, value)?
                    }
                    _ => return Err(GameError::new(GameErrorKind::RepositoryError)),
                };

                player.try_add_stat(stat)?;
            }

            game.add_player(player)?;
        }

        Ok(game)
    }

    /// Deletes a game and all its associated data from the database.
    ///
    /// This method removes the game record, which cascades to delete:
    /// - All players associated with the game
    /// - All stats associated with those players
    ///
    /// The cascade behavior is defined by the database schema's foreign key constraints.
    async fn delete(&self, id: Uuid) -> Result<(), GameError> {
        let id_str = id.to_string();

        let result = sqlx::query!(
            r#"DELETE FROM games WHERE id = ?"#,
            id_str
        )
        .execute(&self.db)
        .await
        .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e)))?;

        if result.rows_affected() == 0 {
            return Err(GameError::new(GameErrorKind::GameNotFound));
        }

        Ok(())
    }
}

// TODO: create tests for update when some more things can be changed in the game entity
#[cfg(test)]
mod tests {
    use crate::domain::game::entities::player::Player;
    use crate::domain::game::error::GameErrorKind;
    use crate::domain::user::entities::User;
    use crate::infrastructure::persistence::db::create_test_pool;
    use super::*;

    async fn insert_users(pool: SqlitePool) {
        let mut conn = pool.acquire().await.unwrap();

        sqlx::query!(
                r#"
                INSERT INTO users (id, name, password)
                VALUES (?, ?, ?), (?, ?, ?), (?, ?, ?)
                "#,
                "04b5b922-1f09-4132-be16-4aabe54d09d2",
                "test-user-name",
                "test-user-password",
                "64338889-ef87-4aa5-b6a7-bc1ae91f2ef8",
                "test-user2-name",
                "test-user2-password",
                "58855217-ebcd-40ce-aebb-c30a235358e8",
                "test-user3-name",
                "test-user3-password",
            )
            .execute(&mut *conn)
            .await
            .map_err(|e| GameError::with_source(GameErrorKind::RepositoryError, Box::new(e))).unwrap();
    }

    fn create_test_game() -> Game {
        let mut game = Game::new(Uuid::new_v4(), "test-game-name".to_string());

        let mut player = Player::new(
            Uuid::new_v4(),
            User::try_new("04b5b922-1f09-4132-be16-4aabe54d09d2".try_into().unwrap(), "test-user-name".to_string(), "test-user-password".to_string()).unwrap(),
        );
        player.try_add_bool_stat(Uuid::new_v4(), "bool-stat".to_string(), true).unwrap();
        player.try_add_number_stat(Uuid::new_v4(), "number-stat".to_string(), 50).unwrap();
        game.add_player(player).unwrap();

        let mut player2 = Player::new(
            Uuid::new_v4(),
            User::try_new("64338889-ef87-4aa5-b6a7-bc1ae91f2ef8".try_into().unwrap(), "test-user2-name".to_string(), "test-user2-password".to_string()).unwrap(),
        );
        player2.try_add_bool_stat(Uuid::new_v4(), "bool-stat".to_string(), false).unwrap();
        player2.try_add_number_stat(Uuid::new_v4(), "number-stat".to_string(), 50).unwrap();
        player2.try_add_string_stat(Uuid::new_v4(), "string-stat".to_string(), "string-value".to_string()).unwrap();
        game.add_player(player2).unwrap();

        let player3 = Player::new(
            Uuid::new_v4(),
            User::try_new("58855217-ebcd-40ce-aebb-c30a235358e8".try_into().unwrap(), "test-user3-name".to_string(), "test-user3-password".to_string()).unwrap(),
        );
        game.add_player(player3).unwrap();

        game
    }

    #[tokio::test]
    async fn save_and_find_by_id() {
        let pool = create_test_pool().await;
        let repo = SqliteGameRepository::new(pool.clone());

        insert_users(pool).await;
        let game = create_test_game();

        let res = repo.save(&game).await;
        assert!(res.is_ok());

        let res = repo.find_by_id(game.id()).await;
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res, game);
    }

    #[tokio::test]
    async fn test_find_by_id_not_found() {
        let pool = create_test_pool().await;
        let repo = SqliteGameRepository::new(pool);

        let res = repo.find_by_id(Uuid::new_v4()).await;
        assert!(res.is_err());
        let res = res.unwrap_err();
        assert_eq!(res, GameError::new(GameErrorKind::GameNotFound));
    }

    #[tokio::test]
    async fn test_save_user_not_found() {
        let pool = create_test_pool().await;
        let repo = SqliteGameRepository::new(pool.clone());

        let game = create_test_game();

        let res = repo.save(&game).await;
        assert!(res.is_err());
        let res = res.unwrap_err();
        assert_eq!(res, GameError::new(GameErrorKind::UserForPlayerNotFound));
    }

    #[tokio::test]
    async fn test_save_and_delete() {
        let pool = create_test_pool().await;
        let repo = SqliteGameRepository::new(pool.clone());

        insert_users(pool).await;
        let game = create_test_game();

        let res = repo.save(&game).await;
        assert!(res.is_ok());

        let res = repo.delete(game.id()).await;
        assert!(res.is_ok());

        let res = repo.find_by_id(game.id()).await;
        assert!(res.is_err());
        let res = res.unwrap_err();
        assert_eq!(res, GameError::new(GameErrorKind::GameNotFound));
    }

    #[tokio::test]
    async fn test_delete_not_found() {
        let pool = create_test_pool().await;
        let repo = SqliteGameRepository::new(pool);

        let res = repo.delete(Uuid::new_v4()).await;
        assert!(res.is_err());
        let res = res.unwrap_err();
        assert_eq!(res, GameError::new(GameErrorKind::GameNotFound));
    }

    #[tokio::test]
    async fn test_save_duplicate_game_name() {
        let pool = create_test_pool().await;
        let repo = SqliteGameRepository::new(pool.clone());

        insert_users(pool).await;
        let game1 = create_test_game();

        let res = repo.save(&game1).await;
        assert!(res.is_ok());

        // Try to save another game with the same name
        let mut game2 = Game::new(Uuid::new_v4(), "test-game-name".to_string());
        let player = Player::new(
            Uuid::new_v4(),
            User::try_new("04b5b922-1f09-4132-be16-4aabe54d09d2".try_into().unwrap(), "test-user-name".to_string(), "test-user-password".to_string()).unwrap(),
        );
        game2.add_player(player).unwrap();

        let res = repo.save(&game2).await;
        assert!(res.is_err());
        let res = res.unwrap_err();
        assert_eq!(res, GameError::new(GameErrorKind::GameAlreadyExists));
    }

    #[tokio::test]
    async fn test_update_not_found() {
        let pool = create_test_pool().await;
        let repo = SqliteGameRepository::new(pool.clone());

        insert_users(pool).await;
        let game = create_test_game();

        let res = repo.update(&game).await;
        assert!(res.is_err());
        let res = res.unwrap_err();
        assert_eq!(res, GameError::new(GameErrorKind::GameNotFound));
    }
}