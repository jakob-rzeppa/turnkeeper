use serde::{Deserialize, Serialize};

use crate::domain::common::identifier::Id;

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
    ChangePlayerOrder { names_in_order: Vec<String> },
    AttachUserToPlayer { player: String, user_id: Id },
    DetachUserFromPlayer { player: String },

    // Actions
    ExecuteAction { action: String, payload: String },

    // Debug
    Debug(String),
}
