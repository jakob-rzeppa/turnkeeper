//! # Game Session
//!
//! A `GameSession` represents an active, in-memory instance of a single game.
//! It owns the [`GameRuntime`] and manages real-time communication with
//! the connected Game Master (GM) and user players over WebSocket connections.
//!
//! ## Lifecycle
//!
//! 1. A session is created via [`GameSession::try_new`], which loads the game
//!    metadata from the repository and initialises the runtime.
//! 2. When the GM opens a WebSocket connection, [`GameSession::gm_connect`] is
//!    called.  The session loops, receiving [`GameCommand`]s from the GM,
//!    applying them via the runtime to the game, and broadcasting the updated game state
//!    to all connected clients.
//! 3. Users can connect via [`GameSession::user_connect`], which follows the
//!    same ticket-based flow. Multiple users can be connected simultaneously.
//! 4. The loop exits when the connection sends a [`ConnectionMessageDto::Close`]
//!    message (or the connection is otherwise dropped), at which point the
//!    stored connection handle is cleared.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::application::game::contracts::{ConnectionContract, GameRepositoryContract};
use crate::application::game::dto::OutgoingConnectionMessageDto;
use crate::application::game::runtime::GameRuntime;
use crate::application::game::session::gm_connection_state::GmConnectionState;
use crate::application::game::session::user_connection_state::UserConnectionState;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::game::commands::GameCommand;
use crate::domain::game::projections::game_error::GameErrorProjection;
use crate::domain::game::value_objects::id::Id;

mod gm_connection_state;
mod gm_lifecycle;
mod user_connection_state;
mod user_lifecycle;

/// How long a ticket remains valid (in seconds).
const TICKET_TTL_SECS: u64 = 30;

/// An active in-memory game session.
///
/// Owns the [`GameRuntime`] and manages connections
/// from the GM and multiple user players.
pub struct GameSession<Connection, GameRepository>
where
    Connection: ConnectionContract,
    GameRepository: GameRepositoryContract
{
    /// The live game runtime that holds all current game state.
    runtime: Arc<RwLock<GameRuntime>>,
    /// The active GM WebSocket connection, if one is currently established.
    gm_connection: Arc<RwLock<GmConnectionState<Connection>>>,
    /// The active user connections, if any are currently established.
    user_connections: Arc<RwLock<HashMap<Id, Arc<RwLock<UserConnectionState<Connection>>>>>>,
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
    /// initializes the in-memory runtime containing the game aggregate.  No GM connection is established
    /// at this point.
    ///
    /// # Errors
    ///
    /// Returns a [`GameError`] if the game cannot be found or the repository
    /// call fails.
    pub async fn try_new(game_id: Id, game_repository: Arc<GameRepository>) -> Result<Self, GameError> {
        let game_metadata = game_repository.get_metadata_by_id(game_id).await?;

        let mut runtime = GameRuntime::new(game_metadata.id, game_metadata.name);

        // Apply all past commands to reconstruct the current game state
        let past_commands = game_repository.get_game_history(game_id).await?;
        for command in past_commands {
            runtime.handle_command(command).map_err(|e| GameError::with_source(GameErrorKind::GameHistoryInvalid, Box::new(e)))?;
        }

        Ok(Self {
            runtime: Arc::new(RwLock::new(runtime)),
            gm_connection: Arc::new(RwLock::new(GmConnectionState::None)),
            user_connections: Arc::new(RwLock::new(HashMap::new())),
            game_repo: game_repository,
        })
    }

    /// Applies a [`GameCommand`] to the runtime and calls broadcast_game_state.
    ///
    /// If the runtime accepts the command successfully, the new state is persisted to the
    /// repository.  If the command is rejected (e.g. due to invalid data or an illegal state
    /// transition) the error is logged to stderr and the game state is not persisted.
    ///
    /// Regardless of outcome the current game state is broadcast to all
    /// connected clients so they remain in sync.
    ///
    /// If the command was triggered by a user action, the `user_id` of the triggering user is passed.
    /// This is used to check if the user has permission to perform the action.
    async fn handle_command(&self, command: GameCommand, _user_id: Option<&Id>) {
        let mut runtime_guard = self.runtime.write().await;

        let res = runtime_guard.handle_command(command.clone());

        match res {
            Ok(_) => {
                // Persist the game state only if the command was handled successfully
                if let Err(e) = self.game_repo.log_command(runtime_guard.get_id(), command).await {
                    eprintln!("Failed to save game state: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Failed to handle command: {}", e);
                self.send_error_to_gm(e).await;
            },
        };

        drop(runtime_guard);

        self.broadcast_game_state().await;
    }

    async fn send_error_to_gm(&self, error: GameError) {
        let gm_conn_guard = self.gm_connection.read().await;
        if let GmConnectionState::Connected { ref connection } = *gm_conn_guard {
            let error_projection = GameErrorProjection::from(error);
            let message = OutgoingConnectionMessageDto::GameError(error_projection);
            connection.send(message).await;
        }
    }

    /// Sends the current [`GmGameInfo`] state to the GM and all connected users.
    async fn broadcast_game_state(&self) {
        let runtime_guard = self.runtime.read().await;

        let gm_conn_guard = self.gm_connection.read().await;
        if let GmConnectionState::Connected { ref connection } = *gm_conn_guard {
            let gm_game_state = runtime_guard.get_gm_game_projection();
            let message = OutgoingConnectionMessageDto::GmGameState(gm_game_state);
            connection.send(message).await;
        }

        let user_connections_guard = self.user_connections.read().await;
        for (user_id, user_connection) in user_connections_guard.iter() {
            let user_connection_guard = user_connection.read().await;
            if let UserConnectionState::Connected { ref connection, .. } = *user_connection_guard {
                let user_game_state = runtime_guard.get_user_game_projection(user_id);
                let message = OutgoingConnectionMessageDto::UserGameInfo(user_game_state);
                connection.send(message).await;
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
            runtime: Arc::clone(&self.runtime),
            gm_connection: Arc::clone(&self.gm_connection),
            user_connections: Arc::clone(&self.user_connections),
            game_repo: Arc::clone(&self.game_repo),
        }
    }
}