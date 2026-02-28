//! # Game Session
//!
//! A `GameSession` represents an active, in-memory instance of a single game.
//! It owns the live [`Game`] aggregate and manages real-time communication with
//! the connected Game Master (GM) and user players over WebSocket connections.
//!
//! ## Lifecycle
//!
//! 1. A session is created via [`GameSession::try_new`], which loads the game
//!    metadata from the repository and initialises the aggregate.
//! 2. When the GM opens a WebSocket connection, [`GameSession::gm_connect`] is
//!    called.  The session loops, receiving [`GameEvent`]s from the GM,
//!    applying them to the aggregate, and broadcasting the updated game state
//!    to all connected clients.
//! 3. Users can connect via [`GameSession::user_connect`], which follows the
//!    same ticket-based flow. Multiple users can be connected simultaneously.
//! 4. The loop exits when the connection sends a [`ConnectionMessageDto::Close`]
//!    message (or the connection is otherwise dropped), at which point the
//!    stored connection handle is cleared.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use crate::application::game::contracts::{ConnectionContract, GameRepositoryContract};
use crate::application::game::session::gm_connection_state::GmConnectionState;
use crate::application::game::session::user_connection_state::UserConnectionState;
use crate::domain::game::entities::game::Game;
use crate::domain::game::error::GameError;
use crate::domain::game::events::GameEvent;
use crate::domain::game::projections::gm_game_info::GmGameInfo;

mod gm_connection_state;
mod gm_lifecycle;
mod user_connection_state;
mod user_lifecycle;

/// How long a ticket remains valid (in seconds).
const TICKET_TTL_SECS: u64 = 30;

    /// An active in-memory game session.
///
/// Owns the [`Game`] aggregate for one game and manages connections
/// from the GM and multiple user players.
pub struct GameSession<Connection, GameRepository>
where
    Connection: ConnectionContract,
    GameRepository: GameRepositoryContract
{
    /// The live game aggregate that holds all current game state.
    game: Arc<RwLock<Game>>,
    /// The active GM WebSocket connection, if one is currently established.
    gm_connection: Arc<RwLock<GmConnectionState<Connection>>>,
    /// The active user connections, if any are currently established.
    user_connections: Arc<RwLock<HashMap<Uuid, Arc<RwLock<UserConnectionState<Connection>>>>>>,
    /// Shared repository used for persistence operations.
    game_repo: Arc<GameRepository>,
}

impl<Connection, GameRepository> GameSession<Connection, GameRepository>
where
    Connection: ConnectionContract,
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
            gm_connection: Arc::new(RwLock::new(GmConnectionState::None)),
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

    /// Sends the current [`GmGameInfo`] state to the GM and all connected users.
    async fn broadcast_game_state(&self) {
        let game_guard = self.game.read().await;
        let game_state = GmGameInfo::from(&*game_guard);
        drop(game_guard);

        let json_game_state = match serde_json::to_string(&game_state) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Failed to serialize game state: {}", e);
                return;
            }
        };

        let gm_conn_guard = self.gm_connection.read().await;
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
}

impl<Connection, GameRepository> Clone for GameSession<Connection, GameRepository>
where
    Connection: ConnectionContract,
    GameRepository: GameRepositoryContract
{
    fn clone(&self) -> Self {
        Self {
            game: Arc::clone(&self.game),
            gm_connection: Arc::clone(&self.gm_connection),
            user_connections: Arc::clone(&self.user_connections),
            game_repo: Arc::clone(&self.game_repo),
        }
    }
}