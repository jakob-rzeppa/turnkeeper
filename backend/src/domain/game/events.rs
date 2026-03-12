//! # Game Events
//!
//! Domain events that can be applied to a [`Game`](super::entities::game::Game) aggregate
//! via its `handle_event` method. Events are serialized as JSON over WebSocket.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    SetNotes(String),
    SetHiddenNotes(String),

    // Player
    AddPlayer { player_id: String },
    ChangePlayerOrder(Vec<String>),

    // Stats
    AddStatToPlayer { player_id: String, stat_key: String, stat_type: String, stat_value: String },
    ChangeStatOfPlayer { player_id: String, stat_id: String, stat_type: String, stat_value: String },
    RemoveStatFromPlayer { player_id: String, stat_id: String },

    // Tradables
    AddTradable { tradable_id: String, name: String, initial_value: f64 },
    RemoveTradable { tradable_id: String },
    ChangePlayerTradableValue { player_id: String, tradable_id: String, new_value: f64 },
    SendTradable { from_id: String, to_id: String, tradable_id: String, amount: f64 },

    AttachUserToPlayer { player_id: String, user_id: String },
    DetachUserFromPlayer { player_id: String },

    // Debug
    Debug(String),
}

impl GameEvent {
    /// Returns whether a user (non-GM) client is allowed to send this event.
    ///
    /// Currently only [`Debug`](GameEvent::Debug) is permitted for users.
    pub fn is_user_permitted(&self, _user_id: &Uuid) -> bool {
        match self {
            GameEvent::Debug(_) => true,
            _ => false
        }
    }
}