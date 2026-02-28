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

use std::sync::Arc;
use std::time::Instant;
use futures_util::SinkExt;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::application::game::contracts::{GameRepositoryContract, GmConnectionContract};
use crate::application::game::dto::ConnectionMessageDto;
use crate::domain::game::entities::game::Game;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::game::events::GameEvent;
use crate::domain::game::projections::GmGameInfo;
use crate::infrastructure::websocket::gm_connection::WebSocketGmConnection;

/// How long a ticket remains valid (in seconds).
const TICKET_TTL_SECS: u64 = 30;

pub enum ConnectionState<GmConnection>
where
    GmConnection: GmConnectionContract
{
    None,
    Pending {
        ticket: String,
        ticket_created_at: Instant,
    },
    Connected {
        connection: GmConnection,
    }
}

impl<GmConnection> ConnectionState<GmConnection>
where
    GmConnection: GmConnectionContract
{
    fn as_mut(&mut self) -> Option<&mut GmConnection> {
        if let ConnectionState::Connected { connection } = self {
            Some(connection)
        } else {
            None
        }
    }
}

/// An active in-memory game session.
///
/// Owns the [`Game`] aggregate for one game and optionally holds an open
/// connection to the GM.
pub struct GameSession<GmConnection, GameRepository>
where
    GmConnection: GmConnectionContract,
    GameRepository: GameRepositoryContract
{
    /// The live game aggregate that holds all current game state.
    game: Arc<RwLock<Game>>,
    /// The active GM WebSocket connection, if one is currently established.
    gm_conn: Arc<RwLock<ConnectionState<GmConnection>>>,
    /// Shared repository used for persistence operations.
    game_repo: Arc<GameRepository>,
}

impl<GmConnection, GameRepository> GameSession<GmConnection, GameRepository>
where
    GmConnection: GmConnectionContract,
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
            gm_conn: Arc::new(RwLock::new(ConnectionState::None)),
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

        let mut gm_conn_guard = self.gm_conn.write().await;
        if let ConnectionState::Connected { ref mut connection } = *gm_conn_guard {
            connection.send(json_game_state).await;
        }
    }

    pub async fn gm_pre_connect(&self) -> Result<String, GameError> {
        let mut gm_conn_guard = self.gm_conn.write().await;

        // Opportunistically clean up expired pending connection
        if let ConnectionState::Pending { ref ticket_created_at, .. } = *gm_conn_guard {
            if ticket_created_at.elapsed().as_secs() >= TICKET_TTL_SECS {
                *gm_conn_guard = ConnectionState::None;
            }
        }

        // Only allow a new pending connection if there is currently no active or pending connection
        match *gm_conn_guard {
            ConnectionState::None => {
                let ticket = Uuid::new_v4().to_string();
                *gm_conn_guard = ConnectionState::Pending {
                    ticket: ticket.clone(),
                    ticket_created_at: Instant::now(),
                };
                Ok(ticket)
            },
            ConnectionState::Pending { .. } => Err(GameError::new(GameErrorKind::GmAlreadyConnected)),
            ConnectionState::Connected { .. } => Err(GameError::new(GameErrorKind::GmAlreadyConnected)),
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
        let mut gm_conn_guard = self.gm_conn.write().await;

        match *gm_conn_guard {
            ConnectionState::Connected { .. } => {
                eprintln!("GM connection already established for this session. Rejecting new connection.");
                return Err(GameError::new(GameErrorKind::GmAlreadyConnected));
            },
            ConnectionState::Pending { ref ticket, ref ticket_created_at } => {
                if ticket.ne(&connection_ticket) || ticket_created_at.elapsed().as_secs() >= TICKET_TTL_SECS {
                    eprintln!("Invalid or expired ticket for GM connection. Rejecting connection.");

                    // Clear the pending state
                    *gm_conn_guard = ConnectionState::None;

                    return Err(GameError::new(GameErrorKind::InvalidConnectionToken));
                }
            },
            ConnectionState::None => {
                eprintln!("No pending GM connection to upgrade. Rejecting connection.");
                return Err(GameError::new(GameErrorKind::NoPendingConnection));
            }
        }

        // At this point we know the ticket is valid, so we can accept the connection
        *gm_conn_guard = ConnectionState::Connected { connection };
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
                let mut guard = self.gm_conn.write().await;
                let conn = guard.as_mut().expect("gm_conn is some");
                conn.recv().await
            };
            // guard dropped — lock released

            match msg {
                ConnectionMessageDto::Event(event) => self.handle_event(event).await,
                _ => break,
            }
        }

        let mut gm_conn_guard = self.gm_conn.write().await;
        println!("Closing GmWebSocket connection.");
        *gm_conn_guard = ConnectionState::None;
        Ok(())
    }
}

impl<GmConnection, GameRepository> Clone for GameSession<GmConnection, GameRepository>
where
    GmConnection: GmConnectionContract,
    GameRepository: GameRepositoryContract
{
    fn clone(&self) -> Self {
        Self {
            game: Arc::clone(&self.game),
            gm_conn: Arc::clone(&self.gm_conn),
            game_repo: Arc::clone(&self.game_repo),
        }
    }
}