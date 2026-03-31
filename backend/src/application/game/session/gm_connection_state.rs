//! # GM Connection State
//!
//! State machine for the GM WebSocket connection within a [`GameSession`](super::GameSession).

use crate::application::common::connection::ConnectionContract;
use crate::application::game::dto::{IncomingConnectionMessageDto, OutgoingConnectionMessageDto};
use crate::application::game::session::TICKET_TTL_SECS;
use crate::domain::game::error::{GameError, GameErrorKind};
use std::sync::Arc;
use std::time::Instant;

/// Tracks the GM connection lifecycle for a game session.
///
/// Transitions: `None` → `Pending` (ticket created) → `Connected` (ticket validated).
pub enum GmConnectionState<Connection>
where
    Connection: ConnectionContract<IncomingConnectionMessageDto, OutgoingConnectionMessageDto>,
{
    /// No GM connection or pending ticket.
    None,
    /// A ticket has been issued; awaiting WebSocket upgrade.
    Pending {
        ticket: String,
        ticket_created_at: Instant,
    },
    /// A GM WebSocket connection is active.
    Connected { connection: Arc<Connection> },
}

impl<Connection> GmConnectionState<Connection>
where
    Connection: ConnectionContract<IncomingConnectionMessageDto, OutgoingConnectionMessageDto>,
{
    /// Returns a cloned Arc to the active connection, or `None`.
    pub fn connection(&self) -> Option<Arc<Connection>> {
        if let GmConnectionState::Connected { connection } = self {
            Some(connection.clone())
        } else {
            None
        }
    }

    /// Validates the ticket and transitions from `Pending` to `Connected`.
    ///
    /// # Errors
    ///
    /// - [`GameErrorKind::GmAlreadyConnected`] — already connected
    /// - [`GameErrorKind::InvalidConnectionToken`] — wrong or expired ticket
    /// - [`GameErrorKind::NoPendingConnection`] — no pending ticket to upgrade
    pub fn upgrade_pending_connection(
        &mut self,
        connection_ticket: String,
        connection: Connection,
    ) -> Result<(), GameError> {
        match self {
            GmConnectionState::Connected { .. } => {
                eprintln!(
                    "GM connection already established for this session. Rejecting new connection."
                );
                Err(GameError::new(GameErrorKind::GmAlreadyConnected))
            }
            GmConnectionState::Pending {
                ticket,
                ticket_created_at,
            } => {
                if connection_ticket.ne(ticket)
                    || ticket_created_at.elapsed().as_secs() >= TICKET_TTL_SECS
                {
                    eprintln!("Invalid or expired ticket for GM connection. Rejecting connection.");

                    // Clear the pending state
                    *self = GmConnectionState::None;

                    Err(GameError::new(GameErrorKind::InvalidConnectionToken))
                } else {
                    *self = GmConnectionState::Connected {
                        connection: Arc::new(connection),
                    };

                    Ok(())
                }
            }
            GmConnectionState::None => {
                eprintln!("No pending GM connection to upgrade. Rejecting connection.");
                Err(GameError::new(GameErrorKind::NoPendingConnection))
            }
        }
    }
}
