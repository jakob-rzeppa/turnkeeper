//! # Game Error
//!
//! Error types for game domain operations.

use std::error::Error;
use std::fmt::{Display, Formatter};

/// Domain error for game-related operations.
///
/// Wraps a [`GameErrorKind`] discriminant and an optional source error for
/// chained error reporting.
#[derive(Debug)]
pub struct GameError {
    pub kind: GameErrorKind,
    source: Option<Box<dyn Error + Send + 'static>>,
}

impl GameError {
    /// Creates a new `GameError` with the given kind and no source.
    pub fn new(kind: GameErrorKind) -> Self {
        GameError { kind, source: None }
    }
    /// Creates a new `GameError` with a source error for chaining.
    pub fn with_source(kind: GameErrorKind, source: Box<dyn Error + Send + 'static>) -> Self {
        GameError { kind, source: Some(source) }
    }
}

impl Error for GameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}

impl PartialEq for GameError {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = self.kind.message();
        write!(f, "{}", msg)?;
        let mut source = self.source();
        while let Some(err) = source {
            write!(f, ": {}", err)?;
            source = err.source();
        }
        Ok(())
    }
}

/// Discriminant for [`GameError`], indicating the category of failure.
#[derive(Debug, PartialEq)]
pub enum GameErrorKind {
    InvalidUuid,
    EmptyStatKey,
    InvalidStat,
    StatNotFound,
    DuplicateStatKey,
    GameAlreadyExists,
    GameSessionAlreadyExists,
    GameHistoryInvalid,
    GameNotFound,
    PlayerNotFound,
    PlayerAlreadyExists,
    UserAlreadyAttachedToAnotherPlayer,
    UserNotInGame,
    RepositoryError,
    UserAlreadyConnected,
    GameSessionCreationFailed,
    GmAlreadyConnected,
    NoPendingConnection,
    InvalidConnectionToken,
    InvalidPlayerOrder,
    TradableAlreadyExists,
    TradableNotFound,
    TradablePlayerNotFound,
    InsufficientTradableValue,
}

impl GameErrorKind {
    pub fn message(&self) -> String {
        match self {
            GameErrorKind::InvalidUuid => "Invalid UUID format".to_string(),
            GameErrorKind::EmptyStatKey => "StatKey is empty".to_string(),
            GameErrorKind::InvalidStat => "Invalid stat".to_string(),
            GameErrorKind::StatNotFound => "Stat not found".to_string(),
            GameErrorKind::DuplicateStatKey => "Duplicate stat key".to_string(),
            GameErrorKind::GameAlreadyExists => "Game already exists".to_string(),
            GameErrorKind::GameNotFound => "Game not found".to_string(),
            GameErrorKind::PlayerNotFound => "Player not found".to_string(),
            GameErrorKind::PlayerAlreadyExists => "Player with same name already exists".to_string(),
            GameErrorKind::RepositoryError => "Unexpected repository error".to_string(),
            GameErrorKind::UserAlreadyConnected => "User already connected to this game".to_string(),
            GameErrorKind::UserNotInGame => "User not in game".to_string(),
            GameErrorKind::GameSessionCreationFailed => "Game session creation failed".to_string(),
            GameErrorKind::GmAlreadyConnected => "GM connection already established for this session".to_string(),
            GameErrorKind::InvalidPlayerOrder => "Invalid player order".to_string(),
            GameErrorKind::NoPendingConnection => "No pending GM connection to upgrade".to_string(),
            GameErrorKind::InvalidConnectionToken => "Invalid or expired token for GM connection".to_string(),
            GameErrorKind::UserAlreadyAttachedToAnotherPlayer => "User is already attached to another player".to_string(),
            GameErrorKind::GameSessionAlreadyExists => "Game session already exists for this game".to_string(),
            GameErrorKind::GameHistoryInvalid => "Game history is invalid".to_string(),
            GameErrorKind::TradableAlreadyExists => "Tradable with same name already exists".to_string(),
            GameErrorKind::TradableNotFound => "Tradable not found".to_string(),
            GameErrorKind::TradablePlayerNotFound => "Player not found in tradable values".to_string(),
            GameErrorKind::InsufficientTradableValue => "Player does not have enough value for this tradable".to_string(),
        }
    }
}