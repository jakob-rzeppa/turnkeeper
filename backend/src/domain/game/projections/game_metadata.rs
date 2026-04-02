//! # Game Metadata Projection
//!
//! Lightweight projection of a game's identity, used for game list endpoints.

use crate::domain::game::value_objects::id::Id;

/// Minimal metadata for a game (ID and display name).
///
/// Returned by the `GET /gm/games` and `GET /user/games` endpoints.
pub struct GameMetadata {
    pub id: Id,
    pub name: String,
    pub gm_user_id: Id,
}
