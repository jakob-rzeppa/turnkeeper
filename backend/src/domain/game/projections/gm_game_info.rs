use serde::Serialize;
use crate::domain::game::entities::game::Game;

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

#[derive(Serialize)]
pub struct GmPlayerInfo {
    pub id: String,

    // The name of the player, which is inherited from the User entity. This is used for display purposes in the frontend.
    pub user: Option<GmPlayerUserInfo>,

    pub stats: Vec<GmStatInfo>,
}

#[derive(Serialize)]
pub struct GmPlayerUserInfo {
    pub id: String,
    pub name: String,
}

#[derive(Serialize)]
pub struct GmStatInfo {
    pub id: String,
    pub key: String,

    // "string", "number", "boolean"
    pub value_type: String,
    pub string_value: Option<String>,
    pub number_value: Option<f64>,
    pub boolean_value: Option<bool>,
}
