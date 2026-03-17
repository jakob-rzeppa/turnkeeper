//! # Game DTOs
//!
//! Data transfer objects for WebSocket message handling.

use crate::domain::game::commands::GameCommand;

#[derive(Debug)]
pub enum ConnectionMessageDto {
    Command(GameCommand),
    Unknown,
    Close,
}