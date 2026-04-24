use serde::{Deserialize, Serialize};

use crate::domain::common::identifier::Identifier;

/// A command that mutates the game aggregate's state.
///
/// Serialized as JSON and sent over WebSocket.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameSessionCommand {
    // On connection, to trigger a sync of the current game state.
    Connect,

    // Turn / Round
    NextTurn,

    // Player
    AddPlayer,
    ChangePlayerOrder(Vec<Identifier>),
    AttachUserToPlayer {
        player_id: Identifier,
        user_id: Identifier,
    },
    DetachUserFromPlayer {
        player_id: Identifier,
    },

    // Actions
    ExecuteAction {
        action_id: Identifier,
        payload: String,
    },

    // Debug
    Debug(String),
}
