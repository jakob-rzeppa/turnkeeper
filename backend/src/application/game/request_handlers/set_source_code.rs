use crate::{
    application::game::{contracts::GameRepositoryContract, error::GameApplicationError},
    domain::common::identifier::Identifier,
};
use std::sync::Arc;

pub struct SetSourceCodeRequestHandler<GameRepository: GameRepositoryContract> {
    repository: Arc<GameRepository>,
}

impl<GameRepository: GameRepositoryContract> SetSourceCodeRequestHandler<GameRepository> {
    pub fn new(repository: Arc<GameRepository>) -> Self {
        Self { repository }
    }

    pub async fn set_source_code(
        &self,
        id: Identifier,
        source_code: String,
    ) -> Result<(), GameApplicationError> {
        let game = self.repository.get_by_id(&id).await?;

        if let Some(mut game) = game {
            game.set_source_code(source_code);
            self.repository.save(&game).await?;
            Ok(())
        } else {
            Err(GameApplicationError::GameNotFound)
        }
    }
}
