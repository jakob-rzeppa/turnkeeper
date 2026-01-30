use std::fmt;

pub struct GameError {
    pub kind: GameErrorKind,
    pub source: Option<Box<GameError>>,
}

#[derive(Debug, PartialEq)]
pub enum GameErrorKind {
    EmptyStatKey,
    InvalidStat,
    DuplicateStatKey,
}

impl GameError {
    pub fn message(&self) -> String {
        match self.kind {
            GameErrorKind::EmptyStatKey => "StatKey is empty".to_string(),
            GameErrorKind::InvalidStat => "Invalid stat".to_string(),
            GameErrorKind::DuplicateStatKey => "Duplicate stat key".to_string(),
        }
    }
}

impl PartialEq for GameError {
    // Only check the error kind, since errors are the same even if they have a different source
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl fmt::Debug for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameError")
            .field("kind", &self.message())
            .field("source", &self.source.as_ref().map(|e| e.to_string()))
            .finish()
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:}", self.message())?;
        let mut source = self.source.as_deref();
        while let Some(err) = source {
            write!(f, ": {}", err)?;
            source = err.source.as_deref();
        }
        Ok(())
    }
}

impl GameError {
    pub fn new(kind: GameErrorKind) -> Self {
        GameError { kind, source: None }
    }

    pub fn with_source(kind: GameErrorKind, source: GameError) -> Self
    {
        GameError {
            kind,
            source: Some(Box::new(source)),
        }
    }
}