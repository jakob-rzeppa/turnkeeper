use serde::Serialize;
use uuid::Uuid;

pub struct GameMetadata {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize)]
pub struct GmGameInfo {
    pub id: String,
    pub name: String,

    pub players: Vec<GmPlayerInfo>,

    pub round_number: u32,
    pub current_player_index: usize,
}

#[derive(Serialize)]
pub struct GmPlayerInfo {
    pub id: String,
    // The name of the player, which is inherited from the User entity. This is used for display purposes in the frontend.
    pub name: String,
    pub stats: Vec<GmStatInfo>,
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