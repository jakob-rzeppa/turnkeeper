use std::time::Instant;
use crate::application::game::contracts::ConnectionContract;
use crate::application::game::session::TICKET_TTL_SECS;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::user::entities::User;

pub enum UserConnectionState<Connection>
where
    Connection: ConnectionContract
{
    None,
    Pending {
        ticket: String,
        ticket_created_at: Instant,
        user: User,
    },
    Connected {
        connection: Connection,
        user: User,
    }
}

impl<Connection> UserConnectionState<Connection>
where
    Connection: ConnectionContract
{
    pub fn connection(&self) -> Option<&Connection> {
        if let UserConnectionState::Connected { connection, .. } = self {
            Some(connection)
        } else {
            None
        }
    }

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
                        connection,
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
