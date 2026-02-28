//! # GmGameInfo Projection
//!
//! Full game state projection that is serialized to JSON and broadcast to all
//! connected clients (GM and users) after every [`GameEvent`](crate::domain::game::events::GameEvent).

use serde::Serialize;
use crate::domain::game::entities::game::Game;

/// Full serializable game state broadcast over WebSocket.
///
/// Sent to all connected clients after each event and on initial connection.
#[derive(Serialize)]
pub struct GmGameInfo {
    pub id: String,
    pub name: String,

    pub players: Vec<GmPlayerInfo>,

    pub round_number: u32,
    pub current_player_index: usize,
}

impl From<&Game> for GmGameInfo {
    fn from(game: &Game) -> Self {
        Self {
            id: game.id().to_string(),
            name: game.name().to_string(),
            players: game.players().iter().map(|p| GmPlayerInfo {
                id: p.id().to_string(),
                user: p.user().map(|u| GmPlayerUserInfo {
                    id: u.id().to_string(),
                    name: u.name().to_string(),
                }),
                stats: p.stats().iter().map(|s| GmStatInfo {
                    id: s.id().to_string(),
                    key: s.key().as_str().to_string(),
                    value_type: s.kind_str().to_string(),
                    string_value: s.as_string().map(|s| s.to_string()),
                    number_value: s.as_number(),
                    boolean_value: s.as_boolean(),
                }).collect(),
            }).collect(),
            round_number: game.round_number(),
            current_player_index: game.current_player_index(),
        }
    }
}

/// Serializable player info within a game state broadcast.
#[derive(Serialize)]
pub struct GmPlayerInfo {
    pub id: String,
    /// The linked user, if any. `None` for anonymous players.
    pub user: Option<GmPlayerUserInfo>,
    pub stats: Vec<GmStatInfo>,
}

/// Serializable user info attached to a player.
#[derive(Serialize)]
pub struct GmPlayerUserInfo {
    pub id: String,
    pub name: String,
}

/// Serializable stat info attached to a player.
#[derive(Serialize)]
pub struct GmStatInfo {
    pub id: String,
    pub key: String,
    /// The type discriminator: `"string"`, `"number"`, or `"boolean"`.
    pub value_type: String,
    pub string_value: Option<String>,
    pub number_value: Option<f64>,
    pub boolean_value: Option<bool>,
}
