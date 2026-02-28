//! # Game Metadata Projection
//!
//! Lightweight projection of a game's identity, used for game list endpoints.

use uuid::Uuid;

/// Minimal metadata for a game (ID and display name).
///
/// Returned by the `GET /gm/games` and `GET /user/games` endpoints.
pub struct GameMetadata {
    pub id: Uuid,
    pub name: String,
}
