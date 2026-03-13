//! # Game Contracts
//!
//! Defines traits (contracts) for game-related infrastructure dependencies.

use crate::application::game::dto::ConnectionMessageDto;
use crate::domain::game::error::GameError;
use crate::domain::game::events::GameEvent;
use crate::domain::game::projections::game_metadata::GameMetadata;
use crate::domain::game::value_objects::id::Id;

/// Repository contract for game data persistence and event sourcing.
///
/// This repository supports:
/// - Game lifecycle management (create, delete)
/// - Metadata queries for game lists
/// - Event sourcing for game state reconstruction
#[mockall::automock]
pub trait GameRepositoryContract {
    /// Creates a new game in the database.
    /// 
    /// This function may be called in a stateless request.
    /// Against clean architecture using this won't require the extra step over the domain, 
    /// to ensure no invariants, since no invariants, that can be checked by the aggregate,
    /// are possible.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the game
    /// * `name` - Display name for the game
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Game created successfully
    /// * `Err(GameError)` - Creation failed (e.g., duplicate Name)
    ///
    /// # Errors
    ///
    /// May return an error if:
    /// - A game with the same Name already exists
    /// - Database connection fails
    /// - Constraint violations occur
    fn create(&self, id: Id, name: String) -> impl Future<Output = Result<(), GameError>> + Send;
    
    /// Retrieves metadata for all games.
    ///
    /// Returns a list of game metadata (ID and name) without full game state.
    /// This is used for displaying game lists in the UI.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<GameMetadata>)` - List of all games (may be empty)
    /// * `Err(GameError)` - Database query failed
    fn get_metadata_all_games(&self) -> impl Future<Output = Result<Vec<GameMetadata>, GameError>> + Send;
    
    /// Retrieves metadata for a specific game.
    ///
    /// # Returns
    ///
    /// * `Ok(GameMetadata)` - Game metadata (ID and name)
    /// * `Err(GameError)` - Game not found or database error
    ///
    /// # Errors
    ///
    /// Returns [`GameErrorKind::GameNotFound`] if no game exists with the given ID.
    fn get_metadata_by_id(&self, id: Id) -> impl Future<Output = Result<GameMetadata, GameError>> + Send;
    
    /// Logs a game event.
    ///
    /// Events are appended to the game's event log and can be replayed later
    /// to reconstruct game state.
    /// 
    /// **Important** this function should be called after the event handling in the game succeeded.
    ///
    /// # Event Sourcing
    ///
    /// Events should be immutable once logged. They form an append-only log
    /// that represents the complete history of the game.
    fn log_event(&self, game_id: Id, event: GameEvent) -> impl Future<Output = Result<(), GameError>> + Send;
    
    /// Retrieves the complete event history for a game.
    ///
    /// Returns all events in chronological order, which can be replayed
    /// to reconstruct the current game state.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<GameEvent>)` - Ordered list of all game events (may be empty for new games)
    /// * `Err(GameError)` - Game not found or database error
    fn get_game_history(&self, id: Id) -> impl Future<Output = Result<Vec<GameEvent>, GameError>> + Send;
    
    /// Deletes a game and all associated data.
    ///
    /// # Cascading Deletes
    ///
    /// Implementations should delete:
    /// - The game metadata
    /// - All event log entries for this game
    ///
    /// # Important
    ///
    /// Ensure the game is not active (no WebSocket connections) before deletion.
    fn delete(&self, game_id: Id) -> impl Future<Output = Result<(), GameError>> + Send;
}

/// Contract for a bidirectional WebSocket connection.
///
/// Abstracts the underlying transport so the session logic can be tested
/// without a real WebSocket.
pub trait ConnectionContract {
    /// Receives the next message from the connection.
    fn recv(&self) -> impl Future<Output = ConnectionMessageDto> + Send;

    /// Sends a JSON string to the connected client.
    fn send(&self, msg: String) -> impl Future<Output = ()> + Send;
}