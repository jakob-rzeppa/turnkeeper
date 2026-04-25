use std::sync::Arc;

use crate::application::game::contracts::GameRepositoryContract;

pub mod create;
pub mod delete;
pub mod get_by_id;
pub mod list_all;
pub mod set_source_code;

pub struct GameRequestHandler<GameRepository: GameRepositoryContract> {
    game_repository: Arc<GameRepository>,
}

impl<GameRepository: GameRepositoryContract> GameRequestHandler<GameRepository> {
    pub fn new(game_repository: Arc<GameRepository>) -> Self {
        Self { game_repository }
    }
}

impl<GameRepository: GameRepositoryContract> Clone for GameRequestHandler<GameRepository> {
    fn clone(&self) -> Self {
        Self {
            game_repository: self.game_repository.clone(),
        }
    }
}
