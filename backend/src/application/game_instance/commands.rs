use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::{common::identifier::Id, game::value_objects::data::Value};

/// A command that mutates the game aggregate's state.
///
/// Serialized as JSON and sent over WebSocket.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameSessionCommand {
    // On connection, to trigger a sync of the current game state.
    Connect,

    // Turn / Round
    AdvanceTurn,

    // Player
    AddPlayer,
    ChangePlayerName { player: String, new_name: String },
    ChangePlayerOrder { names_in_order: Vec<String> },
    AttachUserToPlayer { player: String, user_id: Id },
    DetachUserFromPlayer { player: String },

    // Stats
    ChangeGameStat { stat: String, new_value: Value },
    ChangePlayerStat { player: String, stat: String, new_value: Value },

    // Actions
    ExecuteAction { action: String, params: HashMap<String, Value> }, // params: name -> value

    // Debug
    Debug(String),
}
