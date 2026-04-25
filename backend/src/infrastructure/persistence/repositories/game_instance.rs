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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        application::{
            game::contracts::GameRepositoryContract, user::contracts::UserRepositoryContract,
        },
        domain::{
            common::date_time::DateTime,
            game::{
                entities::{
                    action::Action,
                    game::Game,
                    game_instance::GameInstance,
                    log::Log,
                    page::Page,
                    player::Player,
                    stat::{GameStat, PlayerStat},
                },
                value_objects::{
                    stat_value::StatValue,
                    stat_visibility::{GameStatVisibility, PlayerStatVisibility},
                },
            },
            user::entities::User,
        },
        infrastructure::persistence::{
            db::create_test_pool,
            repositories::{game::SqliteGameRepository, user::SqliteUserRepository},
        },
    };

    use super::*;

    async fn create_and_save_user(user_repo: &SqliteUserRepository, name: String) -> User {
        let user = User::try_new(Identifier::new(), name, "testpassword".to_string()).unwrap();
        user_repo.save(&user).await.unwrap();
        user
    }

    async fn create_and_save_game(game_repo: &SqliteGameRepository) -> Game {
        let game = Game::new_raw(
            Identifier::new(),
            "Test Game".to_string(),
            "A test game description".to_string(),
            "print('Hello, World!')".to_string(),
            crate::domain::common::date_time::DateTime::now(),
            crate::domain::common::date_time::DateTime::now(),
        );
        game_repo.save(&game).await.unwrap();
        game
    }

    async fn create_game_instance(i: u16, game: Game, user1: User, user2: User) -> GameInstance {
        let player_id_1 = Identifier::new();
        let player_id_2 = Identifier::new();

        let test_action = Action::new_raw(
            Identifier::new(),
            "testAction".to_string(),
            "action testAction {}".to_string(),
            5,
        );

        let mut log = Log::new();
        log.log_action(
            user1.id().clone(),
            test_action.id().clone(),
            "Test payload".to_string(),
        );
        log.log_error("Something went wrong.".to_string());
        log.log_system("Hi from the system.".to_string());

        GameInstance::new_raw(
            Identifier::new(),
            format!("Test Game Instance {}", i),
            1,
            5,
            vec![
                GameStat::new_raw(
                    Identifier::new(),
                    "Score".to_string(),
                    StatValue::Int(0),
                    StatValue::Int(100),
                    GameStatVisibility::Public,
                ),
                GameStat::new_raw(
                    Identifier::new(),
                    "Gold".to_string(),
                    StatValue::Int(30),
                    StatValue::Int(5),
                    GameStatVisibility::Private,
                ),
            ],
            vec![
                PlayerStat::new_raw(
                    Identifier::new(),
                    "Health".to_string(),
                    [
                        (player_id_1, StatValue::Int(100)),
                        (player_id_2, StatValue::Int(50)),
                    ]
                    .iter()
                    .cloned()
                    .collect::<HashMap<_, _>>(),
                    StatValue::Int(0),
                    PlayerStatVisibility::Public,
                ),
                PlayerStat::new_raw(
                    Identifier::new(),
                    "Money".to_string(),
                    [
                        (player_id_1, StatValue::Int(20)),
                        (player_id_2, StatValue::Int(10)),
                    ]
                    .iter()
                    .cloned()
                    .collect::<HashMap<_, _>>(),
                    StatValue::Int(0),
                    PlayerStatVisibility::Protected,
                ),
            ],
            vec![],
            vec![Page::new_raw(
                Identifier::new(),
                "testPage".to_string(),
                "page testPage {}".to_string(),
                4,
            )],
            vec![
                Player::new_raw(player_id_1, None),
                Player::new_raw(player_id_2, Some(user1.id().clone())),
            ],
            log,
            game,
            user1.id().clone(),
            DateTime::now(),
            DateTime::now(),
        )
    }

    #[tokio::test]
    async fn test_save_and_get_by_id() {
        let db = create_test_pool().await;
        let game_repo = SqliteGameRepository::new(db.clone());
        let user_repo = SqliteUserRepository::new(db.clone());
        let repository = SqliteGameInstanceRepository::new(db);

        // Create and save users
        let user1 = create_and_save_user(&user_repo, "User 1".to_string()).await;
        let user2 = create_and_save_user(&user_repo, "User 2".to_string()).await;

        // Create and save a game
        let game = create_and_save_game(&game_repo).await;

        // Create a game instance
        let game_instance = create_game_instance(1, game, user1, user2).await;

        // Save the game instance
        repository.save(&game_instance).await.unwrap();

        // Retrieve the game instance by ID
        let retrieved_instance = repository
            .get_by_id(game_instance.id().clone())
            .await
            .unwrap();

        assert!(retrieved_instance.is_some());
        let retrieved_instance = retrieved_instance.unwrap();
        assert_eq!(retrieved_instance, game_instance);
    }

    #[tokio::test]
    async fn test_list_by_game_id() {
        let db = create_test_pool().await;
        let game_repo = SqliteGameRepository::new(db.clone());
        let user_repo = SqliteUserRepository::new(db.clone());
        let repository = SqliteGameInstanceRepository::new(db);

        // Create and save users
        let user1 = create_and_save_user(&user_repo, "User 1".to_string()).await;
        let user2 = create_and_save_user(&user_repo, "User 2".to_string()).await;

        // Create and save a game
        let game = create_and_save_game(&game_repo).await;

        // Create and save multiple game instances
        for i in 0..5 {
            let game_instance =
                create_game_instance(i, game.clone(), user1.clone(), user2.clone()).await;
            repository.save(&game_instance).await.unwrap();
        }

        // List game instances by game ID
        let instances_metadata = repository.list_by_game_id(game.id().clone()).await.unwrap();

        assert_eq!(instances_metadata.len(), 5);
        for i in 0..5 {
            assert!(
                instances_metadata
                    .iter()
                    .any(|meta| meta.name == format!("Test Game Instance {}", i))
            ); // Each instance should have the correct name
        }
        assert!(
            instances_metadata
                .iter()
                .all(|meta| &meta.game_id == game.id())
        ); // All instances should belong to the same game
        assert!(instances_metadata.iter().all(|g_i| g_i.current_round == 5));
        assert!(instances_metadata.iter().all(|g_i| g_i.player_count == 2));

        // Check order
        for i in 0..4 {
            assert!(
                instances_metadata[i].last_played_at >= instances_metadata[i + 1].last_played_at
            ); // Instances should be ordered by last_played_at descending
        }
    }

    #[tokio::test]
    async fn test_delete() {
        let db = create_test_pool().await;
        let game_repo = SqliteGameRepository::new(db.clone());
        let user_repo = SqliteUserRepository::new(db.clone());
        let repository = SqliteGameInstanceRepository::new(db);

        // Create and save users
        let user1 = create_and_save_user(&user_repo, "User 1".to_string()).await;
        let user2 = create_and_save_user(&user_repo, "User 2".to_string()).await;

        // Create and save a game
        let game = create_and_save_game(&game_repo).await;

        // Create a game instance
        let game_instance = create_game_instance(1, game, user1, user2).await;

        // Save the game instance
        repository.save(&game_instance).await.unwrap();

        // Check if the instance exists before deletion
        let retrieved_instance = repository
            .get_by_id(game_instance.id().clone())
            .await
            .unwrap();
        assert!(retrieved_instance.is_some()); // The instance should exist before deletion

        // Delete the game instance
        repository
            .delete(
                game_instance.source_game().id().clone(),
                game_instance.id().clone(),
            )
            .await
            .unwrap();

        // Try to retrieve the deleted game instance by ID
        let retrieved_instance = repository
            .get_by_id(game_instance.id().clone())
            .await
            .unwrap();

        assert!(retrieved_instance.is_none()); // The instance should no longer exist
    }
}
