//! # Game Projections
//!
//! Read-only projections of game state used for serialization and API responses.
//!
//! - [`GameMetadata`](game_metadata::GameMetadata) — lightweight ID + name for game lists
//! - [`GmGameInfo`](gm_game_info::GmGameInfo) — full game state for gm, send over WebSocket
//! - [`UserGameInfo`](user_game_info::UserGameInfo) — limited game info for users, send over WebSocket

pub mod game_metadata;
pub mod gm_game_info;
pub mod user_game_info;
