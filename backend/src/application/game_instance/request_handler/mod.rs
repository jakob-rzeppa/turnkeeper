use std::sync::Arc;

use crate::application::{
    game::{contracts::GameRepositoryContract, root_parser::GameRootParserContract},
    game_instance::contracts::GameInstanceRepositoryContract,
};

pub mod create;
pub mod delete;
pub mod list_by_game;

pub struct GameInstanceRequestHandler {
    game_instance_repository: Arc<dyn GameInstanceRepositoryContract>,
    game_repository: Arc<dyn GameRepositoryContract>,
    game_root_parser: Arc<dyn GameRootParserContract>,
}

impl GameInstanceRequestHandler {
    pub fn new(
        game_instance_repository: Arc<dyn GameInstanceRepositoryContract>,
        game_repository: Arc<dyn GameRepositoryContract>,
        game_root_parser: Arc<dyn GameRootParserContract>,
    ) -> Self {
        Self {
            game_instance_repository,
            game_repository,
            game_root_parser,
        }
    }
}

impl Clone for GameInstanceRequestHandler {
    fn clone(&self) -> Self {
        Self {
            game_instance_repository: self.game_instance_repository.clone(),
            game_repository: self.game_repository.clone(),
            game_root_parser: self.game_root_parser.clone(),
        }
    }
}
