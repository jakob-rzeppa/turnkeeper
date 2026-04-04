use serde::Serialize;

use crate::domain::game::{entities::game::Game, projections::game::user::PlayerGameProjection};

pub mod user;

pub enum GameProjectionVariant {
    ForGm(GameProjection),
    ForUser(PlayerGameProjection),
}

/// Serializable player info within the gm game info.
#[derive(Debug, Serialize, Clone)]
pub struct PlayerProjection {
    pub id: String,
    /// The linked user, if any. `None` for anonymous players.
    pub user_id: Option<String>,
    pub stats: Vec<StatProjection>,
    pub tradables: Vec<TradableProjection>,
}

/// Serializable tradable info within the gm game info.
#[derive(Debug, Serialize, Clone)]
pub struct TradableProjection {
    pub id: String,
    pub name: String,
    pub value: f64,
}

/// Serializable stat info within the gm game info.
#[derive(Debug, Serialize, Clone)]
pub struct StatProjection {
    pub id: String,
    pub key: String,
    /// The type discriminator: `"string"`, `"number"`, or `"boolean"`.
    pub value_type: String,
    pub string_value: Option<String>,
    pub number_value: Option<f64>,
    pub boolean_value: Option<bool>,
}

/// Full serializable game info send to the gm over WebSocket.
#[derive(Debug, Serialize, Clone)]
pub struct GameProjection {
    pub id: String,
    pub name: String,
    pub gm_user_id: String,

    pub players: Vec<PlayerProjection>,

    pub round_number: u32,
    pub current_player_index: usize,

    pub notes: String,
    pub hidden_notes: String,
}

impl From<&Game> for GameProjection {
    fn from(game: &Game) -> Self {
        Self {
            id: game.id().to_string(),
            name: game.name().to_string(),
            gm_user_id: game.gm_user_id().to_string(),
            players: game
                .players()
                .iter()
                .map(|p| PlayerProjection {
                    id: p.id().to_string(),
                    user_id: p.user_id().map(|id| id.to_string()),
                    stats: p
                        .stats()
                        .iter()
                        .map(|s| StatProjection {
                            id: s.id().to_string(),
                            key: s.key().as_str().to_string(),
                            value_type: s.kind_str().to_string(),
                            string_value: s.as_str().map(|s| s.to_string()),
                            number_value: s.as_number(),
                            boolean_value: s.as_boolean(),
                        })
                        .collect(),
                    tradables: game
                        .tradables()
                        .iter()
                        .map(|t| TradableProjection {
                            id: t.id().to_string(),
                            name: t.name().to_string(),
                            value: t
                                .value_for_player(p.id().clone())
                                .expect("there shall never be an invalid state"),
                        })
                        .collect(),
                })
                .collect(),
            round_number: game.round_number(),
            current_player_index: game.current_player_index(),
            notes: game.notes().to_string(),
            hidden_notes: game.hidden_notes().to_string(),
        }
    }
}
