//! # GM Lifecycle
//!
//! Implements the GM connection lifecycle on [`GameSession`]

use std::time::Instant;
use uuid::Uuid;
use crate::application::game::contracts::{ConnectionContract, GameRepositoryContract};
use crate::application::game::dto::IncomingConnectionMessageDto;
use crate::application::game::session::gm_connection_state::GmConnectionState;
use crate::application::game::session::{GameSession, TICKET_TTL_SECS};
use crate::domain::game::error::{GameError, GameErrorKind};

impl<Connection, GameRepository> GameSession<Connection, GameRepository>
where
    Connection: ConnectionContract,
    GameRepository: GameRepositoryContract
{
    /// Creates a single-use GM connection ticket.
    ///
    /// Transitions the GM connection state from `None` to `Pending`.
    /// The ticket expires after [`TICKET_TTL_SECS`] seconds.
    ///
    /// # Errors
    ///
    /// Returns [`GameErrorKind::GmAlreadyConnected`] if a GM connection
    /// or pending ticket already exists.
    pub async fn gm_pre_connect(&self) -> Result<String, GameError> {
        let mut gm_connection_guard = self.gm_connection.write().await;

        // Opportunistically clean up expired pending connection
        if let GmConnectionState::Pending { ref ticket_created_at, .. } = *gm_connection_guard {
            if ticket_created_at.elapsed().as_secs() >= TICKET_TTL_SECS {
                *gm_connection_guard = GmConnectionState::None;
            }
        }

        // Only allow a new pending connection if there is currently no active or pending connection
        match *gm_connection_guard {
            GmConnectionState::None => {
                let ticket = Uuid::new_v4().to_string();
                *gm_connection_guard = GmConnectionState::Pending {
                    ticket: ticket.clone(),
                    ticket_created_at: Instant::now(),
                };
                Ok(ticket)
            },
            GmConnectionState::Pending { .. } => Err(GameError::new(GameErrorKind::GmAlreadyConnected)),
            GmConnectionState::Connected { .. } => Err(GameError::new(GameErrorKind::GmAlreadyConnected)),
        }
    }

    /// Accepts a GM WebSocket connection and drives the session command loop.
    ///
    /// Stores the provided connection handle and then continuously reads
    /// incoming messages:
    ///
    /// - [`ConnectionMessageDto::Command`] — forwarded to [`handle_command`](Self::handle_command).
    /// - [`ConnectionMessageDto::Close`] — breaks the loop and clears the connection.
    ///
    /// This method returns only after the connection is closed. Only one GM
    /// connection may be active per session at a time.
    pub async fn gm_connect(&self, connection_ticket: String, connection: Connection) -> Result<(), GameError> {
        let mut gm_connection_guard = self.gm_connection.write().await;

        gm_connection_guard.upgrade_pending_connection(connection_ticket, connection)?;
        println!("Gm connection established");

        // Drop the gm_conn_guard to avoid holding the write lock while we await messages in the loop below
        drop(gm_connection_guard);

        // Broadcast the initial game state to the newly connected GM
        self.broadcast_game_state().await;

        // Handle incoming messages until the connection is closed.
        // The write lock must be scoped so it is released before handle_command,
        // which calls broadcast_game_state and re-acquires the lock.
        loop {
            // Clone the Arc<Connection> so we can await recv() without holding the read lock
            let conn = {
                let guard = self.gm_connection.read().await;
                guard.connection().expect("gm connection is some")
            };
            // Guard is dropped here, read lock released

            let msg = conn.recv().await;

            match msg {
                IncomingConnectionMessageDto::Command(command) => self.handle_command(command, None).await,
                _ => break,
            }
        }

        let mut gm_connection_guard = self.gm_connection.write().await;
        println!("Closing GmWebSocket connection.");
        *gm_connection_guard = GmConnectionState::None;
        Ok(())
    }
}
