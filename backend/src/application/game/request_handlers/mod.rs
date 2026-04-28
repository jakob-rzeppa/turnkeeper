use std::sync::Arc;

use crate::application::game::{
    contracts::GameRepositoryContract, root_parser::GameRootParserContract,
};

pub mod check_source_code;
pub mod create;
pub mod delete;
pub mod get_by_id;
pub mod list_all;
pub mod set_source_code;

pub struct GameRequestHandler<
    GameRepository: GameRepositoryContract,
    GameRootParser: GameRootParserContract,
> {
    game_repository: Arc<GameRepository>,
    game_root_parser: Arc<GameRootParser>,
}

impl<GameRepository: GameRepositoryContract, GameRootParser: GameRootParserContract>
    GameRequestHandler<GameRepository, GameRootParser>
{
    pub fn new(
        game_repository: Arc<GameRepository>,
        game_root_parser: Arc<GameRootParser>,
    ) -> Self {
        Self {
            game_repository,
            game_root_parser,
        }
    }
}

impl<GameRepository: GameRepositoryContract, GameRootParser: GameRootParserContract> Clone
    for GameRequestHandler<GameRepository, GameRootParser>
{
    fn clone(&self) -> Self {
        Self {
            game_repository: self.game_repository.clone(),
            game_root_parser: self.game_root_parser.clone(),
        }
    }
}
