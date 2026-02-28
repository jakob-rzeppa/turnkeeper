//! # Game Requests
//!
//! Request DTOs for game-related operations.

use uuid::Uuid;

pub struct CreateGameRequest {
    pub name: String,
}

pub struct DeleteGameRequest {
    pub id: Uuid,
}