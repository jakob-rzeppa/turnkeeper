use serde::Serialize;
use crate::domain::game::entities::game::Game;
use crate::domain::game::value_objects::id::Id;

/// Serializable player info within the gm game info.
#[derive(Serialize)]
pub struct GmPlayerInfo {
    pub id: Id,
    /// The linked user, if any. `None` for anonymous players.
    pub user_id: Option<Id>,
    pub stats: Vec<GmStatInfo>,
    pub tradables: Vec<GmTradableInfo>,
}

/// Serializable tradable info within the gm game info.
#[derive(Serialize)]
pub struct GmTradableInfo {
    pub id: Id,
    pub name: String,
    pub value: f64,
}

/// Serializable stat info within the gm game info.
#[derive(Serialize)]
pub struct GmStatInfo {
    pub id: Id,
    pub key: String,
    /// The type discriminator: `"string"`, `"number"`, or `"boolean"`.
    pub value_type: String,
    pub string_value: Option<String>,
    pub number_value: Option<f64>,
    pub boolean_value: Option<bool>,
}

/// Full serializable game info send to the gm over WebSocket.
#[derive(Serialize)]
pub struct GmGameInfo {
    pub id: Id,
    pub name: String,

    pub players: Vec<GmPlayerInfo>,

    pub round_number: u32,
    pub current_player_index: usize,

    pub notes: String,
    pub hidden_notes: String,
}

impl From<&Game> for GmGameInfo {
    fn from(game: &Game) -> Self {
        Self {
            id: *game.id(),
            name: game.name().to_string(),
            players: game.players().iter().map(|p| GmPlayerInfo {
                id: *p.id(),
                user_id: p.user_id(),
                stats: p.stats().iter().map(|s| GmStatInfo {
                    id: *s.id(),
                    key: s.key().as_str().to_string(),
                    value_type: s.kind_str().to_string(),
                    string_value: s.as_string().map(|s| s.to_string()),
                    number_value: s.as_number(),
                    boolean_value: s.as_boolean(),
                }).collect(),
                tradables: game.tradables().iter().map(|t| {
                    GmTradableInfo {
                        id: *t.id(),
                        name: t.name().to_string(),
                        value: t.value_for_player(p.id().clone()).expect("there shall never be an invalid state"),
                    }
                }).collect(),
            }).collect(),
            round_number: game.round_number(),
            current_player_index: game.current_player_index(),
            notes: game.notes().to_string(),
            hidden_notes: game.hidden_notes().to_string(),
        }
    }
}