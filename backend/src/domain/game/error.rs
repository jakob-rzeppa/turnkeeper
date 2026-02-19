use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct GameError {
    pub kind: GameErrorKind,
    source: Option<Box<dyn Error + Send + 'static>>,
}

impl GameError {
    pub fn new(kind: GameErrorKind) -> Self {
        GameError { kind, source: None }
    }
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

#[derive(Debug, PartialEq)]
pub enum GameErrorKind {
    EmptyStatKey,
    InvalidStat,
    DuplicateStatKey,
    GameAlreadyExists,
    GameNotFound,
    PlayerWithSameNameAlreadyExists,
    RepositoryError,
    UserForPlayerNotFound,
    GameSessionCreationFailed,
}

impl GameErrorKind {
    pub fn message(&self) -> String {
        match self {
            GameErrorKind::EmptyStatKey => "StatKey is empty".to_string(),
            GameErrorKind::InvalidStat=> "Invalid stat".to_string(),
            GameErrorKind::DuplicateStatKey => "Duplicate stat key".to_string(),
            GameErrorKind::GameAlreadyExists => "Game already exists".to_string(),
            GameErrorKind::GameNotFound => "Game not found".to_string(),
            GameErrorKind::PlayerWithSameNameAlreadyExists => "Player with same name already exists".to_string(),
            GameErrorKind::RepositoryError => "Unexpected repository error".to_string(),
            GameErrorKind::UserForPlayerNotFound => "User not found in game".to_string(),
            GameErrorKind::GameSessionCreationFailed => "Game session creation failed".to_string(),
        }
    }
}