use std::sync::Arc;

use crate::application::{
    common::parser::GameParserContract, game::contracts::GameRepositoryContract, game_instance::contracts::GameInstanceRepositoryContract
};

pub mod check_source_code;
pub mod create;
pub mod delete;
pub mod get_by_id;
pub mod list_all;
pub mod set_source_code;

pub struct GameRequestHandler {
    game_repository: Arc<dyn GameRepositoryContract>,
    game_instance_repository: Arc<dyn GameInstanceRepositoryContract>,
    game_root_parser: Arc<dyn GameParserContract>,
}

impl GameRequestHandler {
    pub fn new(
        game_repository: Arc<dyn GameRepositoryContract>,
        game_instance_repository: Arc<dyn GameInstanceRepositoryContract>,
        game_root_parser: Arc<dyn GameParserContract>,
    ) -> Self {
        Self {
            game_repository,
            game_instance_repository,
            game_root_parser,
        }
    }
}

impl Clone for GameRequestHandler {
    fn clone(&self) -> Self {
        Self {
            game_repository: self.game_repository.clone(),
            game_instance_repository: self.game_instance_repository.clone(),
            game_root_parser: self.game_root_parser.clone(),
        }
    }
}
