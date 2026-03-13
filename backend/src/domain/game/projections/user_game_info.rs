use serde::Serialize;
use uuid::Uuid;
use crate::domain::game::entities::game::Game;
use crate::domain::game::value_objects::id::Id;

/// Serializable player info within the gm game info.
#[derive(Serialize)]
pub struct UserPlayerInfo {
    pub id: String,
    /// The linked user, if any. `None` for anonymous players.
    pub user_id: Option<String>,
}

/// Serializable player info within the gm game info.
#[derive(Serialize)]
pub struct UserOwnPlayerInfo {
    pub id: String,
    /// The linked user.
    pub user_id: String,
    pub stats: Vec<UserStatInfo>,
    pub tradables: Vec<UserTradableInfo>,
}


/// Serializable tradable info within the user game info.
#[derive(Serialize)]
pub struct UserTradableInfo {
    pub id: String,
    pub name: String,
    pub value: f64,
}

/// Serializable stat info within the gm game info.
#[derive(Serialize)]
pub struct UserStatInfo {
    pub id: String,
    pub key: String,
    /// The type discriminator: `"string"`, `"number"`, or `"boolean"`.
    pub value_type: String,
    pub string_value: Option<String>,
    pub number_value: Option<f64>,
    pub boolean_value: Option<bool>,
}

/// Limited serializable game info send to the users over WebSocket.
#[derive(Serialize)]
pub struct UserGameInfo {
    pub id: String,
    pub name: String,

    pub own_player: Option<UserOwnPlayerInfo>,

    pub players: Vec<UserPlayerInfo>,

    pub round_number: u32,
    pub current_player_index: usize,

    pub notes: String,
}

impl UserGameInfo {
    pub fn for_user(game: &Game, user_id: &Id) -> Self {
        let own_player = game.players().iter()
            .find(|p| p.user_id() == Some(*user_id))
            .map(|p| {
                UserOwnPlayerInfo {
                    id: p.id().to_string(),
                    user_id: p.user_id().unwrap().to_string(),
                    stats: p.stats().iter().map(|s| UserStatInfo {
                        id: s.id().to_string(),
                        key: s.key().as_str().to_string(),
                        value_type: s.kind_str().to_string(),
                        string_value: s.as_string().map(|s| s.to_string()),
                        number_value: s.as_number(),
                        boolean_value: s.as_boolean(),
                    }).collect(),
                    tradables: game.tradables().iter().map(|t| { UserTradableInfo {
                        id: t.id().to_string(),
                        name: t.name().to_string(),
                        value: t.value_for_player(p.id().clone()).expect("there shall be no invalid state"),
                    }}).collect(),
                }
            });

        Self {
            id: game.id().to_string(),
            name: game.name().to_string(),
            own_player,
            players: game.players().iter().map(|p| UserPlayerInfo {
                id: p.id().to_string(),
                user_id: p.user_id().map(|u| u.to_string()),
            }).collect(),
            round_number: game.round_number(),
            current_player_index: game.current_player_index(),
            notes: game.notes().to_string(),
        }
    }
}