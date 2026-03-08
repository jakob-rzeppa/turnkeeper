//! # User Lifecycle
//!
//! Implements the user connection lifecycle on [`GameSession`]:
//! ticket creation ([`user_pre_connect`](GameSession::user_pre_connect)) and
//! the event-loop ([`user_connect`](GameSession::user_connect)).
//!
//! Multiple users can be connected simultaneously, unlike the GM which
//! is limited to a single connection.

use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::application::game::contracts::{ConnectionContract, GameRepositoryContract};
use crate::application::game::dto::ConnectionMessageDto;
use crate::application::game::session::user_connection_state::UserConnectionState;
use crate::application::game::session::{GameSession, TICKET_TTL_SECS};
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::user::entities::User;

impl<Connection, GameRepository> GameSession<Connection, GameRepository>
where
    Connection: ConnectionContract,
    GameRepository: GameRepositoryContract
{
    /// Creates a single-use connection ticket for a user.
    ///
    /// If the user has no existing state in this session, a new entry is
    /// created. Transitions the user's connection state to `Pending`.
    ///
    /// # Errors
    ///
    /// Returns [`GameErrorKind::UserAlreadyConnected`] if the user already
    /// has a pending ticket or active connection.
    pub async fn user_pre_connect(&self, user: User) -> Result<String, GameError> {
        let user_connections_guard = self.user_connections.read().await;

        if let Some(user_connection) = user_connections_guard.get(user.id()) {
            let mut user_connection_guard = user_connection.write().await;
            // Opportunistically clean up expired pending connection
            if let UserConnectionState::Pending { ref ticket_created_at, .. } = *user_connection_guard {
                if ticket_created_at.elapsed().as_secs() >= TICKET_TTL_SECS {
                    *user_connection.write().await = UserConnectionState::None;
                }
            }

            // Only allow a new pending connection if there is currently no active or pending connection
            match *user_connection_guard {
                UserConnectionState::None => {
                    let ticket = Uuid::new_v4().to_string();
                    *user_connection_guard = UserConnectionState::Pending {
                        ticket: ticket.clone(),
                        ticket_created_at: Instant::now(),
                        user,
                    };
                    Ok(ticket)
                },
                UserConnectionState::Pending { .. } => Err(GameError::new(GameErrorKind::UserAlreadyConnected)),
                UserConnectionState::Connected { .. } => Err(GameError::new(GameErrorKind::UserAlreadyConnected)),
            }
        } else {
            let ticket = Uuid::new_v4().to_string();
            let user_connection_state = UserConnectionState::Pending {
                ticket: ticket.clone(),
                ticket_created_at: Instant::now(),
                user: user.clone(),
            };
            drop(user_connections_guard);

            let mut user_connections_guard = self.user_connections.write().await;
            user_connections_guard.insert(*user.id(), Arc::new(RwLock::new(user_connection_state)));
            Ok(ticket)
        }
    }

    /// Accepts a user WebSocket connection and drives the session event loop.
    ///
    /// Validates the ticket, transitions to `Connected`, broadcasts the
    /// current game state, then enters a receive loop identical to
    /// [`gm_connect`](Self::gm_connect). On disconnect the user entry is
    /// removed from the session.
    pub async fn user_connect(&self, user_id: Uuid, connection_ticket: String, connection: Connection) -> Result<(), GameError> {
        let user_connections_guard = self.user_connections.read().await;

        let user_connection = match user_connections_guard.get(&user_id) {
            Some(conn) => conn,
            None => {
                eprintln!("No pending connection found for user_id {}. Rejecting connection.", user_id);
                return Err(GameError::new(GameErrorKind::NoPendingConnection));
            }
        };

        let mut user_connection_guard = user_connection.write().await;

        user_connection_guard.upgrade_pending_connection(connection_ticket, connection).map_err(|err| {
            eprintln!("Failed to upgrade connection for user_id {}. Rejecting connection.", user_id);
            err
        })?;
        println!("User connection established");

        // Drop the guards to avoid holding the locks while we await messages in the loop below
        drop(user_connection_guard);
        drop(user_connections_guard);

        // Broadcast the initial game state to the newly connected user
        self.broadcast_game_state().await;

        // Handle incoming messages until the connection is closed.
        // The write lock must be scoped so it is released before handle_event,
        // which calls broadcast_game_state and re-acquires the lock.
        loop {
            // Clone the Arc<Connection> so we can await recv() without holding the read lock
            let conn = {
                let user_connections_guard = self.user_connections.read().await;
                let user_connection = user_connections_guard.get(&user_id).expect("user connection should exist");
                let user_connection_guard = user_connection.read().await;
                user_connection_guard.connection().expect("user connection is some")
            };
            // Guard is dropped here, read lock released

            let msg = conn.recv().await;

            match msg {
                ConnectionMessageDto::Event(event) => {
                    self.handle_event(event, Some(&user_id)).await
                }
                _ => break,
            }
        }

        let mut user_connections_guard = self.user_connections.write().await;
        println!("Closing user WebSocket connection.");
        user_connections_guard.remove(&user_id);
        Ok(())
    }
}
