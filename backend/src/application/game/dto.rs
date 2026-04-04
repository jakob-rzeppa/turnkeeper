//! # Game DTOs
//!
//! Data transfer objects for WebSocket message handling.

use std::fmt::Display;

use crate::{
    application::game::commands::GameCommand,
    domain::game::projections::{
        game::{GameProjection, user::PlayerGameProjection},
        game_error::GameErrorProjection,
    },
};

#[derive(Debug)]
pub enum IncomingConnectionMessageDto {
    Command(GameCommand),
    Unknown,
}

#[derive(Debug, Clone)]
pub enum OutgoingConnectionMessageDto {
    FullGameProjection(GameProjection),
    PlayerGameProjection(PlayerGameProjection),
    GameError(GameErrorProjection),
}

impl Display for OutgoingConnectionMessageDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutgoingConnectionMessageDto::FullGameProjection(info) => {
                write!(
                    f,
                    "FullGameProjection {}",
                    serde_json::to_string(info).unwrap()
                )
            }
            OutgoingConnectionMessageDto::PlayerGameProjection(info) => {
                write!(
                    f,
                    "PlayerGameProjection {}",
                    serde_json::to_string(info).unwrap()
                )
            }
            OutgoingConnectionMessageDto::GameError(error) => {
                write!(f, "GameError {}", serde_json::to_string(error).unwrap())
            }
        }
    }
}
