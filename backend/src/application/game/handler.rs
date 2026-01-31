use uuid::Uuid;
use crate::application::game::dto::GameOverviewDto;
use crate::domain::game::error::GameError;

/// The GameHandler is the central service handling all the games.
pub struct GameHandler {}

impl GameHandler {
    pub fn get_games_overview() -> Result<Vec<GameOverviewDto>, GameError> {
        panic!("Not implemented yet")
    }

    pub fn create_game() -> Result<Uuid, GameError> {
        panic!("Not implemented yet")
    }

    pub fn delete_game(id: Uuid) -> Result<(), GameError> {
        panic!("Not implemented yet")
    }
}