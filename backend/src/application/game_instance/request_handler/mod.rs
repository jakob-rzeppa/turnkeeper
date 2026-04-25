use std::sync::Arc;

use crate::application::{
    game::contracts::GameRepositoryContract,
    game_instance::contracts::GameInstanceRepositoryContract,
};

pub mod create;
pub mod delete;
pub mod list_by_game;

pub struct GameInstanceRequestHandler<
    GameInstanceRepository: GameInstanceRepositoryContract,
    GameRepository: GameRepositoryContract,
> {
    game_instance_repository: Arc<GameInstanceRepository>,
    game_repository: Arc<GameRepository>,
}

impl<GameInstanceRepository: GameInstanceRepositoryContract, GameRepository: GameRepositoryContract>
    GameInstanceRequestHandler<GameInstanceRepository, GameRepository>
{
    pub fn new(
        game_instance_repository: Arc<GameInstanceRepository>,
        game_repository: Arc<GameRepository>,
    ) -> Self {
        Self {
            game_instance_repository,
            game_repository,
        }
    }
}

impl<GameInstanceRepository: GameInstanceRepositoryContract, GameRepository: GameRepositoryContract>
    Clone for GameInstanceRequestHandler<GameInstanceRepository, GameRepository>
{
    fn clone(&self) -> Self {
        Self {
            game_instance_repository: self.game_instance_repository.clone(),
            game_repository: self.game_repository.clone(),
        }
    }
}
