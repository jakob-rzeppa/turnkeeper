use std::sync::Arc;
use crate::application::game::contracts::GameRepositoryContract;
use crate::application::game::responses::OverviewGameResponse;
use crate::domain::game::error::GameError;

pub struct GameGetOverviewRequestHandler<GameRepository: GameRepositoryContract> {
    repository: Arc<GameRepository>,
}

impl<GameRepository: GameRepositoryContract> GameGetOverviewRequestHandler<GameRepository> {
    pub fn new(repository: Arc<GameRepository>) -> Self { Self { repository } }
    
    pub async fn get_overview(&self) -> Result<OverviewGameResponse, GameError> {
        Ok(OverviewGameResponse {
            games_metadata: self.repository.get_metadata_all_games().await?
        })
    }
}