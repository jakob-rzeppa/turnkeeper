//! # User Connection State
//!
//! State machine for a single user's WebSocket connection within a [`GameSession`](super::GameSession).

use std::sync::Arc;
use std::time::Instant;
use crate::application::game::contracts::ConnectionContract;
use crate::application::game::session::TICKET_TTL_SECS;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::user::entities::User;

/// Tracks one user's connection lifecycle for a game session.
///
/// Transitions: `None` → `Pending` (ticket created) → `Connected` (ticket validated).
/// Multiple users can be connected to the same session simultaneously.
pub enum UserConnectionState<Connection>
where
    Connection: ConnectionContract
{
    /// No connection or pending ticket for this user.
    None,
    /// A ticket has been issued; awaiting WebSocket upgrade.
    Pending {
        ticket: String,
        ticket_created_at: Instant,
        user: User,
    },
    /// A user WebSocket connection is active.
    Connected {
        connection: Arc<Connection>,
        user: User,
    }
}

impl<Connection> UserConnectionState<Connection>
where
    Connection: ConnectionContract
{
    /// Returns a cloned Arc to the active connection, or `None`.
    pub fn connection(&self) -> Option<Arc<Connection>> {
        if let UserConnectionState::Connected { connection, .. } = self {
            Some(connection.clone())
        } else {
            None
        }
    }

    /// Validates the ticket and transitions from `Pending` to `Connected`.
    ///
    /// # Errors
    ///
    /// - [`GameErrorKind::UserAlreadyConnected`] — already connected
    /// - [`GameErrorKind::InvalidConnectionToken`] — wrong or expired ticket
    /// - [`GameErrorKind::NoPendingConnection`] — no pending ticket to upgrade
    pub fn upgrade_pending_connection(&mut self, connection_ticket: String, connection: Connection) -> Result<(), GameError> {
        match self {
            UserConnectionState::Connected { .. } => {
                eprintln!("User connection already established for this session. Rejecting new connection.");
                Err(GameError::new(GameErrorKind::UserAlreadyConnected))
            },
            UserConnectionState::Pending { ticket, ticket_created_at, user } => {
                if connection_ticket.ne(ticket) || ticket_created_at.elapsed().as_secs() >= TICKET_TTL_SECS {
                    eprintln!("Invalid or expired ticket for user connection. Rejecting connection.");

                    // Clear the pending state
                    *self = UserConnectionState::None;

                    Err(GameError::new(GameErrorKind::InvalidConnectionToken))
                } else {
                    *self = UserConnectionState::Connected {
                        connection: Arc::new(connection),
                        user: user.clone(),
                    };

                    Ok(())
                }
            },
            UserConnectionState::None => {
                eprintln!("No pending user connection to upgrade. Rejecting connection.");
                Err(GameError::new(GameErrorKind::NoPendingConnection))
            }
        }
    }
}
