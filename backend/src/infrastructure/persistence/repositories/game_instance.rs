use sqlx::SqlitePool;

use crate::{
    application::{
        common::error::DatabaseError, game_instance::contracts::GameInstanceRepositoryContract,
    },
    domain::{
        common::identifier::Identifier,
        game::{
            entities::game_instance::GameInstance,
            projections::game_instance_metadata::GameInstanceMetadataProjection,
        },
    },
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
    async fn list_by_game_id(
        &self,
        game_id: Identifier,
    ) -> Result<Vec<GameInstanceMetadataProjection>, DatabaseError> {
        let game_instances = sqlx::query!(
            r#"
            SELECT serialized FROM game_instances
            "#
        )
        .fetch_all(&self.db)
        .await
        .map_err(|e| {
            DatabaseError::Unknown(format!(
                "Database error while listing GameInstances for game {}: {}",
                game_id, e
            ))
        })?;

        let metadata_list = game_instances
            .into_iter()
            .map(|row| -> Result<GameInstance, DatabaseError> {
                serde_json::from_str(&row.serialized).map_err(|e| {
                    DatabaseError::DeserializationError(format!(
                        "Failed to deserialize GameInstance for game {}: {}",
                        game_id, e
                    ))
                })
            })
            .filter(|game_instance| match game_instance {
                Ok(instance) => instance.source_game().id() == &game_id,
                Err(_) => true,
            })
            .map(|game_instance| {
                let game_instance: GameInstance = game_instance?;
                Ok(game_instance.get_metadata_projection())
            })
            .collect::<Result<Vec<GameInstanceMetadataProjection>, DatabaseError>>()?;

        Ok(metadata_list)
    }

    async fn get_by_id(&self, id: Identifier) -> Result<Option<GameInstance>, DatabaseError> {
        let id_str = id.to_string();

        let row = sqlx::query!(
            r#"
            SELECT serialized FROM game_instances
            WHERE id = ?
            "#,
            id_str
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|e| {
            DatabaseError::Unknown(format!(
                "Database error while fetching GameInstance with id {}: {}",
                id, e
            ))
        })?;

        if let Some(row) = row {
            let game_instance: GameInstance =
                serde_json::from_str(&row.serialized).map_err(|e| {
                    DatabaseError::DeserializationError(format!(
                        "Failed to deserialize GameInstance with id {}: {}",
                        id, e
                    ))
                })?;
            Ok(Some(game_instance))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, game_instance: &GameInstance) -> Result<(), DatabaseError> {
        let id = game_instance.id().to_string();
        let serialized = serde_json::to_string(game_instance).map_err(|e| {
            DatabaseError::SerializationError(format!(
                "Failed to serialize GameInstance with id {}: {}",
                id, e
            ))
        })?;

        sqlx::query!(
            r#"
            INSERT INTO game_instances (id, serialized)
            VALUES (?, ?)
            ON CONFLICT(id) DO UPDATE SET serialized = excluded.serialized
            "#,
            id,
            serialized
        )
        .execute(&self.db)
        .await
        .map_err(|e| {
            DatabaseError::Unknown(format!(
                "Database error while saving GameInstance with id {}: {}",
                id, e
            ))
        })?;

        Ok(())
    }

    async fn delete(
        &self,
        _game_id: Identifier,
        instance_id: Identifier,
    ) -> Result<(), DatabaseError> {
        let id_str = instance_id.to_string();

        sqlx::query!(
            r#"
            DELETE FROM game_instances
            WHERE id = ?
            "#,
            id_str
        )
        .execute(&self.db)
        .await
        .map_err(|e| {
            DatabaseError::Unknown(format!(
                "Database error while deleting GameInstance with id {}: {}",
                instance_id, e
            ))
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        application::{
            game::contracts::GameRepositoryContract,
            game_instance::contracts::GameInstanceRepositoryContract,
            user::contracts::UserRepositoryContract,
        },
        domain::{
            common::{date_time::DateTime, identifier::Identifier, position::Position},
            game::{
                entities::{
                    game::Game,
                    game_instance::GameInstance,
                    weak::{
                        action::Action,
                        log::Log,
                        page::Page,
                        player::Player,
                        stat::{GameStat, PlayerStat},
                    },
                },
                value_objects::{
                    data::{VariableType, VariableValue},
                    visibility::{
                        ActionVisibility, GameStatVisibility, PageVisibility, PlayerStatVisibility,
                    },
                },
            },
            user::entities::User,
        },
        infrastructure::persistence::{
            db::create_test_pool,
            repositories::{
                game::SqliteGameRepository, game_instance::SqliteGameInstanceRepository,
                user::SqliteUserRepository,
            },
        },
        util::unordered_equal::equals_unordered,
    };

    fn get_random_string() -> String {
        Identifier::new().to_string()
    }

    fn create_user() -> User {
        User::try_new(Identifier::new(), get_random_string(), get_random_string()).unwrap()
    }

    fn create_player() -> Player {
        Player::new()
    }

    fn create_player_with_user(user_id: &Identifier) -> Player {
        Player::new_raw(get_random_string(), Some(user_id.clone()))
    }

    fn create_game_stat() -> GameStat {
        GameStat::new(
            get_random_string(),
            VariableType::Int,
            VariableValue::Int(0),
            GameStatVisibility::Public,
            Position::new(1, 1),
        )
    }

    fn create_player_stat(player_name_1: String, player_name_2: String) -> PlayerStat {
        let mut values = std::collections::HashMap::new();
        values.insert(player_name_1, VariableValue::Int(0));
        values.insert(player_name_2, VariableValue::Int(0));

        PlayerStat::new_raw(
            get_random_string(),
            VariableType::Int,
            values,
            VariableValue::Int(5),
            PlayerStatVisibility::Public,
            Position::new(1, 1),
        )
    }

    fn create_action() -> Action {
        Action::new(
            get_random_string(),
            vec![],
            vec![],
            ActionVisibility::Public,
            "fn execute() {}".to_string(),
            Position::new(1, 1),
        )
    }

    fn create_page() -> Page {
        Page::new(
            get_random_string(),
            PageVisibility::Public,
            "# Page Content".to_string(),
            Position::new(1, 1),
        )
    }

    fn create_log() -> Log {
        Log::new()
    }

    fn create_game() -> Game {
        Game::new_raw(
            Identifier::new(),
            get_random_string(),
            get_random_string(),
            get_random_string(),
            DateTime::now(),
            DateTime::now(),
        )
    }

    fn create_game_instance(game: &Game, gm_user: &User, player_user: &User) -> GameInstance {
        let player1 = create_player_with_user(player_user.id());
        let player2 = create_player();

        GameInstance::new_raw(
            Identifier::new(),
            get_random_string(),
            1,
            5,
            vec![create_game_stat(), create_game_stat()],
            vec![
                create_player_stat(player1.name().to_string(), player2.name().to_string()),
                create_player_stat(player2.name().to_string(), player1.name().to_string()),
            ],
            vec![create_action()],
            vec![create_page()],
            vec![player1, player2],
            create_log(),
            game.clone(),
            gm_user.id().clone(),
            DateTime::now(),
            DateTime::now(),
        )
    }

    async fn create_repositories() -> (
        SqliteGameInstanceRepository,
        SqliteGameRepository,
        SqliteUserRepository,
    ) {
        let db = create_test_pool().await;
        (
            SqliteGameInstanceRepository::new(db.clone()),
            SqliteGameRepository::new(db.clone()),
            SqliteUserRepository::new(db.clone()),
        )
    }

    #[tokio::test]
    async fn test_save_and_get_game_instance() {
        let (game_instance_repo, game_repo, user_repo) = create_repositories().await;
        let gm_user = create_user();
        let player_user = create_user();
        user_repo.save(&gm_user).await.unwrap();
        user_repo.save(&player_user).await.unwrap();
        let game = create_game();
        game_repo.save(&game).await.unwrap();

        let game_instance = create_game_instance(&game, &gm_user, &player_user);
        game_instance_repo.save(&game_instance).await.unwrap();

        let retrieved_instance = game_instance_repo
            .get_by_id(game_instance.id().clone())
            .await
            .unwrap();

        assert!(retrieved_instance.is_some());
        let retrieved_instance = retrieved_instance.unwrap();
        assert_eq!(retrieved_instance, game_instance);
    }

    #[tokio::test]
    async fn test_delete_game_instance() {
        let (game_instance_repo, game_repo, user_repo) = create_repositories().await;
        let gm_user = create_user();
        let player_user = create_user();
        user_repo.save(&gm_user).await.unwrap();
        user_repo.save(&player_user).await.unwrap();
        let game = create_game();
        game_repo.save(&game).await.unwrap();

        let game_instance = create_game_instance(&game, &gm_user, &player_user);
        game_instance_repo.save(&game_instance).await.unwrap();

        let retrieved_instance = game_instance_repo
            .get_by_id(game_instance.id().clone())
            .await
            .unwrap();
        assert!(retrieved_instance.is_some());

        game_instance_repo
            .delete(game.id().clone(), game_instance.id().clone())
            .await
            .unwrap();

        let retrieved_instance = game_instance_repo
            .get_by_id(game_instance.id().clone())
            .await
            .unwrap();

        assert!(retrieved_instance.is_none());
    }

    #[tokio::test]
    async fn test_list_by_game_id() {
        let (game_instance_repo, game_repo, user_repo) = create_repositories().await;
        let gm_user = create_user();
        let player_user = create_user();
        user_repo.save(&gm_user).await.unwrap();
        user_repo.save(&player_user).await.unwrap();
        let game = create_game();
        game_repo.save(&game).await.unwrap();

        let game_instance1 = create_game_instance(&game, &gm_user, &player_user);
        let game_instance2 = create_game_instance(&game, &gm_user, &player_user);
        game_instance_repo.save(&game_instance1).await.unwrap();
        game_instance_repo.save(&game_instance2).await.unwrap();

        let projections = game_instance_repo
            .list_by_game_id(game.id().clone())
            .await
            .unwrap();
        assert_eq!(projections.len(), 2);

        assert!(equals_unordered(
            &vec![
                game_instance1.get_metadata_projection(),
                game_instance2.get_metadata_projection()
            ],
            &projections
        ));
    }
}
