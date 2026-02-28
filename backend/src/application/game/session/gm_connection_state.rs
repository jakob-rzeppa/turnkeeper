use std::time::Instant;
use sqlx::Connection;
use crate::application::game::contracts::ConnectionContract;
use crate::application::game::session::TICKET_TTL_SECS;
use crate::domain::game::error::{GameError, GameErrorKind};

pub enum GmConnectionState<Connection>
where
    Connection: ConnectionContract
{
    None,
    Pending {
        ticket: String,
        ticket_created_at: Instant,
    },
    Connected {
        connection: Connection,
    }
}

impl<Connection> GmConnectionState<Connection>
where
    Connection: ConnectionContract
{
    pub fn connection(&self) -> Option<&Connection> {
        if let GmConnectionState::Connected { connection } = self {
            Some(connection)
        } else {
            None
        }
    }

    pub fn upgrade_pending_connection(&mut self, connection_ticket: String, connection: Connection) -> Result<(), GameError> {
        match self {
            GmConnectionState::Connected { .. } => {
                eprintln!("GM connection already established for this session. Rejecting new connection.");
                Err(GameError::new(GameErrorKind::GmAlreadyConnected))
            },
            GmConnectionState::Pending { ticket, ticket_created_at } => {
                if connection_ticket.ne(ticket) || ticket_created_at.elapsed().as_secs() >= TICKET_TTL_SECS {
                    eprintln!("Invalid or expired ticket for GM connection. Rejecting connection.");

                    // Clear the pending state
                    *self = GmConnectionState::None;

                    Err(GameError::new(GameErrorKind::InvalidConnectionToken))
                } else {
                    *self = GmConnectionState::Connected { connection };

                    Ok(())
                }
            },
            GmConnectionState::None => {
                eprintln!("No pending GM connection to upgrade. Rejecting connection.");
                Err(GameError::new(GameErrorKind::NoPendingConnection))
            }
        }
    }
}
