use crate::application::game::contracts::GameRepositoryContract;
use crate::application::game::requests::OverviewGameResponse;
use crate::domain::game::error::GameError;

pub struct GameGetOverviewRequestHandler<GameRepository: GameRepositoryContract + 'static> {
    repository: GameRepository,
}

impl<GameRepository: GameRepositoryContract + 'static> GameGetOverviewRequestHandler<GameRepository> {
    pub fn new(repository: GameRepository) -> Self { Self { repository } }
    
    pub async fn get_overview(&self) -> Result<OverviewGameResponse, GameError> {
        Ok(OverviewGameResponse {
            games_metadata: self.repository.get_all_games_metadata().await?
        })
    }
}