//! # Game Projections
//!
//! Read-only projections of game state used for serialization and API responses.
//!
//! - [`GameMetadata`](game_metadata::GameMetadata) — lightweight ID + name for game lists
//! - [`GmGameInfo`](gm_game_info::GmGameInfo) — full game state broadcast over WebSocket

pub mod game_metadata;
pub mod gm_game_info;
