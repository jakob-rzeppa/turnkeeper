use serde::Serialize;

use crate::domain::game::error::GameError;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GameErrorProjection {
    pub message: String,
}

impl From<GameError> for GameErrorProjection {
    fn from(error: GameError) -> Self {
        GameErrorProjection { message: error.to_string() }
    }
}