use std::cmp::PartialEq;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct GmError {
    pub kind: GmErrorKind,
    source: Option<Box<dyn Error + 'static>>,
}

impl GmError {
    pub fn new(kind: GmErrorKind) -> Self {
        GmError { kind, source: None }
    }
    pub fn with_source(kind: GmErrorKind, source: Box<dyn Error + 'static>) -> Self {
        GmError { kind, source: Some(source) }
    }
}

impl Error for GmError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

impl PartialEq for GmError {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Display for GmError {
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
pub enum GmErrorKind {
    InvalidCredentials,
    Unauthorized,
    JwtGenerationError,
}

impl GmErrorKind {
    pub fn message(&self) -> String {
        match self {
            GmErrorKind::InvalidCredentials => "Invalid credentials".to_string(),
            GmErrorKind::Unauthorized => "Unauthorized".to_string(),
            GmErrorKind::JwtGenerationError => "JWT generation failed".to_string(),
        }
    }
}