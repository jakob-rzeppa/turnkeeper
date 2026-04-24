//! # Game Overview Handler
//!
//! Returns metadata for all games.

use crate::{
    application::game::{contracts::GameRepositoryContract, error::GameApplicationError},
    domain::game::projections::game::GameProjection,
};
use std::sync::Arc;

pub struct GameGetByIdResponse {
    pub game: GameProjection,
}

pub struct GameGetByIdRequestHandler<GameRepository: GameRepositoryContract> {
    repository: Arc<GameRepository>,
}

impl<GameRepository: GameRepositoryContract> GameGetByIdRequestHandler<GameRepository> {
    pub fn new(repository: Arc<GameRepository>) -> Self {
        Self { repository }
    }

    pub async fn get_game_by_id(
        &self,
        id: String,
    ) -> Result<GameGetByIdResponse, GameApplicationError> {
        let game = self.repository.get_by_id(id.into()).await?;

        if let Some(game) = game {
            Ok(GameGetByIdResponse {
                game: game.get_projection(),
            })
        } else {
            Err(GameApplicationError::GameNotFound)
        }
    }
}
