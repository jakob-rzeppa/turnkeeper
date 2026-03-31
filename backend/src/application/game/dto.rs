//! # Game DTOs
//!
//! Data transfer objects for WebSocket message handling.

use std::fmt::Display;

use crate::{
    application::game::commands::GameCommand,
    domain::game::projections::{
        game_error::GameErrorProjection, gm_game_info::GmGameInfo, user_game_info::UserGameInfo,
    },
};

#[derive(Debug)]
pub enum IncomingConnectionMessageDto {
    Command(GameCommand),
    Unknown,
}

pub enum OutgoingConnectionMessageDto {
    GmGameState(GmGameInfo),
    UserGameInfo(UserGameInfo),
    GameError(GameErrorProjection),
}

impl Display for OutgoingConnectionMessageDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutgoingConnectionMessageDto::GmGameState(info) => {
                write!(f, "GameInfo {}", serde_json::to_string(info).unwrap())
            }
            OutgoingConnectionMessageDto::UserGameInfo(info) => {
                write!(f, "GameInfo {}", serde_json::to_string(info).unwrap())
            }
            OutgoingConnectionMessageDto::GameError(error) => {
                write!(f, "GameError {}", serde_json::to_string(error).unwrap())
            }
        }
    }
}
