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
pub enum IncomingMessageDto {
    Command(GameCommand),
    Unknown,
}

#[derive(Debug, Clone)]
pub enum OutgoingMessageDto {
    GameInstance(GameProjection),
    PlayerGameProjection(PlayerGameProjection),
    GameError(GameErrorProjection),
}

impl Display for OutgoingMessageDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutgoingMessageDto::FullGameProjection(info) => {
                write!(
                    f,
                    "FullGameProjection {}",
                    serde_json::to_string(info).unwrap()
                )
            }
            OutgoingMessageDto::PlayerGameProjection(info) => {
                write!(
                    f,
                    "PlayerGameProjection {}",
                    serde_json::to_string(info).unwrap()
                )
            }
            OutgoingMessageDto::GameError(error) => {
                write!(f, "GameError {}", serde_json::to_string(error).unwrap())
            }
        }
    }
}
