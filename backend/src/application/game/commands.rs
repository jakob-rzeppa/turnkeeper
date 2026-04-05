//! # Game Commands
//!
//! Domain commands that can be applied to a [`Game`](super::entities::game::Game) aggregate
//! via its `handle_command` method. Commands are serialized as JSON over WebSocket.

use crate::domain::game::value_objects::id::Id;
use serde::{Deserialize, Serialize};

/// A command that mutates the game aggregate's state.
///
/// Serialized as JSON and sent over WebSocket by both GM and user clients.
///
/// # Note
///
/// [`is_user_permitted`](GameCommand::is_user_permitted) indicates whether a user
/// (non-GM) client should be allowed to send this command, but it is **not
/// currently enforced** by the session command loop.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameCommand {
    // On connection, to trigger a sync of the current game state.
    Connect,

    // Turn / Round
    NextTurn,
    PreviousTurn,
    SkipTurnToPlayer {
        player_id: Id,
    },

    // Notes
    SetNotes(String),
    SetHiddenNotes(String),

    // Player
    AddPlayer {
        player_id: Id,
    },
    ChangePlayerOrder(Vec<Id>),

    // Stats
    AddStatToPlayer {
        player_id: Id,
        stat_id: Id,
        stat_key: String,
        stat_type: String,
        stat_value: String,
    },
    ChangeStatOfPlayer {
        player_id: Id,
        stat_id: Id,
        stat_type: String,
        stat_value: String,
    },
    RemoveStatFromPlayer {
        player_id: Id,
        stat_id: Id,
    },

    // Tradables
    AddTradable {
        tradable_id: Id,
        name: String,
        initial_value: f64,
    },
    RemoveTradable {
        tradable_id: Id,
    },
    ChangePlayerTradableValue {
        player_id: Id,
        tradable_id: Id,
        new_value: f64,
    },
    SendTradable {
        from_id: Id,
        to_id: Id,
        tradable_id: Id,
        amount: f64,
    },

    AttachUserToPlayer {
        player_id: Id,
        user_id: Id,
    },
    DetachUserFromPlayer {
        player_id: Id,
    },

    // Debug
    Debug(String),
}

impl GameCommand {
    /// Returns whether a user (non-GM) client is allowed to send this command.
    ///
    /// Currently only [`Debug`](GameCommand::Debug) is permitted for users.
    pub fn is_user_permitted(&self, _user_id: &Id) -> bool {
        match self {
            GameCommand::Debug(_) => true,
            _ => false,
        }
    }
}
