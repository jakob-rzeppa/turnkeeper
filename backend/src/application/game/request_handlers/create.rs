//! # Create Game Handler
//!
//! Creates a new game and persists it via the repository.

use std::sync::Arc;
use crate::application::game::contracts::GameRepositoryContract;
use crate::application::game::requests::CreateGameRequest;
use crate::domain::game::error::GameError;
use crate::domain::game::value_objects::id::Id;

pub struct CreateGameRequestHandler<GameRepository: GameRepositoryContract> {
    repository: Arc<GameRepository>,
}

impl<GameRepository: GameRepositoryContract> CreateGameRequestHandler<GameRepository> {
    pub fn new(repository: Arc<GameRepository>) -> Self { Self { repository } }

    /// Creates a game with a generated UUID and returns the new ID.
    pub async fn create_game(&self, request: CreateGameRequest) -> Result<Id, GameError> {
        let id = Id::new();

        self.repository.create(id.clone(), request.name).await?;

        Ok(id)
    }
}