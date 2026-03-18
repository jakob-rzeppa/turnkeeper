//! # Game Requests
//!
//! Request DTOs for game-related operations.

use crate::domain::game::value_objects::id::Id;

pub struct CreateGameRequest {
    pub name: String,
}

pub struct DeleteGameRequest {
    pub id: Id,
}