//! # Game Session
//!
//! A `GameSession` represents an active, in-memory instance of a single game.
//! It owns the live [`Game`] aggregate and manages real-time communication with
//! the connected Game Master (GM) and Players over a WebSocket connection.
//!
//! ## Lifecycle
//!
//! 1. A session is created via [`GameSession::try_new`], which loads the game
//!    metadata from the repository and initialises the aggregate.
//! 2. When the GM opens a WebSocket connection, [`GameSession::gm_connect`] is
//!    called.  The session loops, receiving [`GameEvent`]s from the GM,
//!    applying them to the aggregate, and broadcasting the updated game state
//!    back.
//! 3. The loop exits when the connection sends a [`ConnectionMessageDto::Close`]
//!    message (or the connection is otherwise dropped), at which point the
//!    stored GM connection handle is cleared.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::application::game::contracts::{GameRepositoryContract, GmConnectionContract, UserConnectionContract};
use crate::application::game::dto::ConnectionMessageDto;
use crate::domain::game::entities::game::Game;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::game::events::GameEvent;
use crate::domain::user::entities::User;

/// How long a ticket remains valid (in seconds).
const TICKET_TTL_SECS: u64 = 30;

pub enum GmConnectionState<Connection>
where
    Connection: GmConnectionContract
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
    Connection: GmConnectionContract
{
    fn as_ref(&self) -> Option<&Connection> {
        if let GmConnectionState::Connected { connection } = self {
            Some(connection)
        } else {
            None
        }
    }
}

pub enum UserConnectionState<Connection>
where
    Connection: UserConnectionContract
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
    Connection: UserConnectionContract
{
    fn as_ref(&self) -> Option<&Connection> {
        if let UserConnectionState::Connected { connection, .. } = self {
            Some(connection)
        } else {
            None
        }
    }

    fn user(&self) -> Option<&User> {
        if let UserConnectionState::Connected { user, .. } = self {
            Some(user)
        } else {
            None
        }
    }

    fn upgrade_pending_connection(&mut self, connection_ticket: String, connection: Connection) -> Result<(), GameError> {
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

/// An active in-memory game session.
///
/// Owns the [`Game`] aggregate for one game and optionally holds an open
/// connection to the GM.
pub struct GameSession<GmConnection, UserConnection, GameRepository>
where
    GmConnection: GmConnectionContract,
    UserConnection: UserConnectionContract,
    GameRepository: GameRepositoryContract
{
    /// The live game aggregate that holds all current game state.
    game: Arc<RwLock<Game>>,
    /// The active GM WebSocket connection, if one is currently established.
    gm_connections: Arc<RwLock<GmConnectionState<GmConnection>>>,
    /// The active user connections, if any are currently established.
    user_connections: Arc<RwLock<HashMap<Uuid, Arc<RwLock<UserConnectionState<UserConnection>>>>>>,
    /// Shared repository used for persistence operations.
    game_repo: Arc<GameRepository>,
}

impl<GmConnection, UserConnection, GameRepository> GameSession<GmConnection, UserConnection, GameRepository>
where
    GmConnection: GmConnectionContract,
    UserConnection: UserConnectionContract,
    GameRepository: GameRepositoryContract
{
    /// Creates a new `GameSession` for the given game.
    ///
    /// Fetches the game's metadata (ID and name) from the repository and
    /// initializes the in-memory aggregate.  No GM connection is established
    /// at this point.
    ///
    /// # Errors
    ///
    /// Returns a [`GameError`] if the game cannot be found or the repository
    /// call fails.
    pub async fn try_new(game_id: Uuid, game_repository: Arc<GameRepository>) -> Result<Self, GameError> {
        let game_metadata = game_repository.get_metadata_by_id(game_id).await?;

        let game = Game::new(game_metadata.id, game_metadata.name);

        Ok(Self {
            game: Arc::new(RwLock::new(game)),
            gm_connections: Arc::new(RwLock::new(GmConnectionState::None)),
            user_connections: Arc::new(RwLock::new(HashMap::new())),
            game_repo: game_repository,
        })
    }

    /// Applies a [`GameEvent`] to the aggregate and calls broadcast_game_state.
    ///
    /// If the aggregate accepts the event successfully, the new state is persisted to the
    /// repository.  If the event is rejected (e.g. due to invalid data or an illegal state
    /// transition) the error is logged to stderr and the game state is not persisted.
    ///
    /// Regardless of outcome the current game state is broadcast to all
    /// connected clients so they remain in sync.
    async fn handle_event(&self, event: GameEvent) {
        let mut game_guard = self.game.write().await;

        let res = game_guard.handle_event(event.clone());

        if res.is_ok() {
            // Persist the game state only if the event was handled successfully
            // if let Err(e) = self.game_repo.log_event(event).await {
            //     eprintln!("Failed to save game state: {}", e);
            // }
        } else {
            // TODO: Revert the game state to the previous valid state if the event was rejected.
            // TODO: Send error to gm
            eprintln!("Failed to handle event: {}", res.err().unwrap());
        }

        drop(game_guard);

        self.broadcast_game_state().await;
    }

    async fn broadcast_game_state(&self) {
        let game_guard = self.game.read().await;
        let game_state = game_guard.get_game_info();
        drop(game_guard);

        let json_game_state = match serde_json::to_string(&game_state) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Failed to serialize game state: {}", e);
                return;
            }
        };

        let gm_conn_guard = self.gm_connections.read().await;
        if let GmConnectionState::Connected { ref connection } = *gm_conn_guard {
            connection.send(json_game_state.clone()).await;
        }

        let user_connections_guard = self.user_connections.read().await;
        for user_connection in user_connections_guard.values() {
            let user_connection_guard = user_connection.read().await;
            if let UserConnectionState::Connected { ref connection, .. } = *user_connection_guard {
                connection.send(json_game_state.clone()).await;
            }
        }
    }

    pub async fn gm_pre_connect(&self) -> Result<String, GameError> {
        let mut gm_conn_guard = self.gm_connections.write().await;

        // Opportunistically clean up expired pending connection
        if let GmConnectionState::Pending { ref ticket_created_at, .. } = *gm_conn_guard {
            if ticket_created_at.elapsed().as_secs() >= TICKET_TTL_SECS {
                *gm_conn_guard = GmConnectionState::None;
            }
        }

        // Only allow a new pending connection if there is currently no active or pending connection
        match *gm_conn_guard {
            GmConnectionState::None => {
                let ticket = Uuid::new_v4().to_string();
                *gm_conn_guard = GmConnectionState::Pending {
                    ticket: ticket.clone(),
                    ticket_created_at: Instant::now(),
                };
                Ok(ticket)
            },
            GmConnectionState::Pending { .. } => Err(GameError::new(GameErrorKind::GmAlreadyConnected)),
            GmConnectionState::Connected { .. } => Err(GameError::new(GameErrorKind::GmAlreadyConnected)),
        }
    }

    /// Accepts a GM WebSocket connection and drives the session event loop.
    ///
    /// Stores the provided connection handle and then continuously reads
    /// incoming messages:
    ///
    /// - [`ConnectionMessageDto::Event`] — forwarded to [`handle_event`](Self::handle_event).
    /// - [`ConnectionMessageDto::Close`] — breaks the loop and clears the connection.
    ///
    /// This method returns only after the connection is closed. Only one GM
    /// connection may be active per session at a time.
    pub async fn gm_connect(&self, connection_ticket: String, connection: GmConnection) -> Result<(), GameError> {
        let mut gm_conn_guard = self.gm_connections.write().await;

        match *gm_conn_guard {
            GmConnectionState::Connected { .. } => {
                eprintln!("GM connection already established for this session. Rejecting new connection.");
                return Err(GameError::new(GameErrorKind::GmAlreadyConnected));
            },
            GmConnectionState::Pending { ref ticket, ref ticket_created_at } => {
                if ticket.ne(&connection_ticket) || ticket_created_at.elapsed().as_secs() >= TICKET_TTL_SECS {
                    eprintln!("Invalid or expired ticket for GM connection. Rejecting connection.");

                    // Clear the pending state
                    *gm_conn_guard = GmConnectionState::None;

                    return Err(GameError::new(GameErrorKind::InvalidConnectionToken));
                }
            },
            GmConnectionState::None => {
                eprintln!("No pending GM connection to upgrade. Rejecting connection.");
                return Err(GameError::new(GameErrorKind::NoPendingConnection));
            }
        }

        // At this point we know the ticket is valid, so we can accept the connection
        *gm_conn_guard = GmConnectionState::Connected { connection };
        println!("Gm connection established");

        // Drop the gm_conn_guard to avoid holding the write lock while we await messages in the loop below
        drop(gm_conn_guard);

        // Broadcast the initial game state to the newly connected GM
        self.broadcast_game_state().await;

        // Handle incoming messages until the connection is closed.
        // The write lock must be scoped so it is released before handle_event,
        // which calls broadcast_game_state and re-acquires the lock.
        loop {
            let msg = {
                let guard = self.gm_connections.read().await;
                let conn = guard.as_ref().expect("gm_conn is some");
                conn.recv().await
            };
            // guard dropped — lock released

            match msg {
                ConnectionMessageDto::Event(event) => self.handle_event(event).await,
                _ => break,
            }
        }

        let mut gm_conn_guard = self.gm_connections.write().await;
        println!("Closing GmWebSocket connection.");
        *gm_conn_guard = GmConnectionState::None;
        Ok(())
    }

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

    pub async fn user_connect(&self, user_id: Uuid, connection_ticket: String, connection: UserConnection) -> Result<(), GameError> {
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
            eprintln!("No pending connection found for user_id {}. Rejecting connection.", user_id);
            err
        })?;
        println!("Gm connection established");

        // Drop the guards to avoid holding the locks while we await messages in the loop below
        drop(user_connection_guard);
        drop(user_connections_guard);

        // Broadcast the initial game state to the newly connected GM
        self.broadcast_game_state().await;

        // Handle incoming messages until the connection is closed.
        // The write lock must be scoped so it is released before handle_event,
        // which calls broadcast_game_state and re-acquires the lock.
        loop {
            let msg = {
                let user_connections_guard = self.user_connections.read().await;
                let user_connection = user_connections_guard.get(&user_id).expect("user connection should exist");
                let user_connection_guard = user_connection.read().await;
                let conn = user_connection_guard.as_ref().expect("gm_conn is some");
                conn.recv().await
            };
            // guard dropped — lock released

            match msg {
                ConnectionMessageDto::Event(event) => self.handle_event(event).await,
                _ => break,
            }
        }

        let mut user_connections_guard = self.user_connections.write().await;
        println!("Closing GmWebSocket connection.");
        user_connections_guard.remove(&user_id);
        Ok(())
    }
}

impl<GmConnection, UserConnection, GameRepository> Clone for GameSession<GmConnection, UserConnection, GameRepository>
where
    GmConnection: GmConnectionContract,
    UserConnection: UserConnectionContract,
    GameRepository: GameRepositoryContract
{
    fn clone(&self) -> Self {
        Self {
            game: Arc::clone(&self.game),
            gm_connections: Arc::clone(&self.gm_connections),
            user_connections: Arc::clone(&self.user_connections),
            game_repo: Arc::clone(&self.game_repo),
        }
    }
}