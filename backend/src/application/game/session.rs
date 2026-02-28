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
use uuid::Uuid;
use crate::application::game::contracts::{GameRepositoryContract, GmConnectionContract};
use crate::application::game::dto::ConnectionMessageDto;
use crate::domain::game::entities::game::Game;
use crate::domain::game::error::{GameError, GameErrorKind};
use crate::domain::game::events::GameEvent;
use crate::domain::game::projections::GmGameInfo;

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
    game: Game,
    /// The active GM WebSocket connection, if one is currently established.
    gm_conn: Option<GmConnection>,
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
            game,
            gm_conn: None,
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
    async fn handle_event(&mut self, event: GameEvent) {
        let res = self.game.handle_event(event.clone());

        if res.is_ok() {
            // Persist the game state only if the event was handled successfully
            // if let Err(e) = self.game_repo.log_event(event).await {
            //     eprintln!("Failed to save game state: {}", e);
            // }
        } else {
            // TODO: Send error to gm
            eprintln!("Failed to handle event: {}", res.err().unwrap());
        }

        self.broadcast_game_state().await;
    }

    async fn broadcast_game_state(&mut self) {
        if let Some(gm_conn) = &mut self.gm_conn {
            match serde_json::to_string(&GmGameInfo::from(&self.game)) {
                Ok(json) => gm_conn.send(json).await,
                Err(e) => eprintln!("failed to serialize GmGameInfo: {}", e),
            }
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
    pub async fn gm_connect(&mut self, gm_conn: GmConnection) -> Result<(), GameError> {
        if self.gm_conn.is_some() {
            eprintln!("GM connection already established for this session. Rejecting new connection.");
            return Err(GameError::new(GameErrorKind::GmAlreadyConnected));
        }

        println!("Gm connection established");
        self.gm_conn = Some(gm_conn);

        while let ConnectionMessageDto::Event(event) = self.gm_conn.as_mut().expect("gm_conn is some").recv().await {
            self.handle_event(event).await;
        }

        println!("Closing GmWebSocket connection.");
        self.gm_conn = None;
        Ok(())
    }
}
