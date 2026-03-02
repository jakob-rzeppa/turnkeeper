//! # Game Events
//!
//! Domain events that can be applied to a [`Game`](super::entities::game::Game) aggregate
//! via its `handle_event` method. Events are serialized as JSON over WebSocket.

use serde::{Deserialize, Serialize};

/// An event that mutates the game aggregate's state.
///
/// Serialized as JSON and sent over WebSocket by both GM and user clients.
///
/// # Note
///
/// [`is_user_permitted`](GameEvent::is_user_permitted) indicates whether a user
/// (non-GM) client should be allowed to send this event, but it is **not
/// currently enforced** by the session event loop.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameEvent {
    // Player
    AddPlayer,
    ChangePlayerOrder(Vec<String>),
    AddStatToPlayer { player_id: String, stat_key: String, stat_type: String, stat_value: String },
    ChangeStatOfPlayer { player_id: String, stat_id: String, stat_type: String, stat_value: String },
    RemoveStatFromPlayer { player_id: String, stat_id: String },
    AttachUserToPlayer { player_id: String, user_id: String },
    DetachUserFromPlayer { player_id: String },

    // Debug
    Debug(String),
}

impl GameEvent {
    /// Returns whether a user (non-GM) client is allowed to send this event.
    ///
    /// Currently only [`Debug`](GameEvent::Debug) is permitted for users.
    ///
    /// **Note:** This check is defined but not yet enforced in the session event loop.
    pub fn is_user_permitted(&self) -> bool {
        match self {
            GameEvent::Debug(_) => true,
            _ => false
        }
    }
}