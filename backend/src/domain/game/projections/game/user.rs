use crate::domain::game::{entities::game::Game, value_objects::id::Id};
use serde::Serialize;

/// Serializable player projection within the user game projection.
#[derive(Debug, Serialize, Clone)]
pub struct UserPlayerProjection {
    pub id: String,
    /// The linked user, if any. `None` for anonymous players.
    pub user_id: Option<String>,
}

/// Serializable player projection within the user game projection.
#[derive(Debug, Serialize, Clone)]
pub struct UserOwnPlayerProjection {
    pub id: String,
    /// The linked user.
    pub user_id: String,
    pub stats: Vec<UserStatProjection>,
    pub tradables: Vec<UserTradableProjection>,
}

/// Serializable tradable projection within the user game projection.
#[derive(Debug, Serialize, Clone)]
pub struct UserTradableProjection {
    pub id: String,
    pub name: String,
    pub value: f64,
}

/// Serializable stat projection within the user game projection.
#[derive(Debug, Serialize, Clone)]
pub struct UserStatProjection {
    pub id: String,
    pub key: String,
    /// The type discriminator: `"string"`, `"number"`, or `"boolean"`.
    pub value_type: String,
    pub string_value: Option<String>,
    pub number_value: Option<f64>,
    pub boolean_value: Option<bool>,
}

/// Limited serializable game projection send to the users over WebSocket.
#[derive(Debug, Serialize, Clone)]
pub struct PlayerGameProjection {
    pub id: String,
    pub name: String,
    pub gm_user_id: String,

    pub own_player: Option<UserOwnPlayerProjection>,

    pub players: Vec<UserPlayerProjection>,

    pub round_number: u32,
    pub current_player_index: usize,

    pub notes: String,
}

impl PlayerGameProjection {
    pub fn for_player_user_id(game: &Game, user_id: &Id) -> Self {
        let own_player = game
            .players()
            .iter()
            .find(|p| p.user_id() == Some(*user_id))
            .map(|p| UserOwnPlayerProjection {
                id: p.id().to_string(),
                user_id: p.user_id().unwrap().to_string(),
                stats: p
                    .stats()
                    .iter()
                    .map(|s| UserStatProjection {
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
                    .map(|t| UserTradableProjection {
                        id: t.id().to_string(),
                        name: t.name().to_string(),
                        value: t
                            .value_for_player(p.id().clone())
                            .expect("there shall be no invalid state"),
                    })
                    .collect(),
            });

        Self {
            id: game.id().to_string(),
            name: game.name().to_string(),
            gm_user_id: game.gm_user_id().to_string(),
            own_player,
            players: game
                .players()
                .iter()
                .map(|p| UserPlayerProjection {
                    id: p.id().to_string(),
                    user_id: p.user_id().map(|u| u.to_string()),
                })
                .collect(),
            round_number: game.round_number(),
            current_player_index: game.current_player_index(),
            notes: game.notes().to_string(),
        }
    }
}
