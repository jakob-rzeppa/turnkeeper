use std::cmp::PartialEq;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct UserError {
    pub kind: UserErrorKind,
    source: Option<Box<dyn Error + 'static>>,
}

impl UserError {
    pub fn new(kind: UserErrorKind) -> Self {
        UserError { kind, source: None }
    }
    pub fn with_source(kind: UserErrorKind, source: Box<dyn Error + 'static>) -> Self {
        UserError { kind, source: Some(source) }
    }
}

impl Error for UserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

impl PartialEq for UserError {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl Display for UserError {
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
pub enum UserErrorKind {
    EmptyName,
    PasswordTooShort {
        required: usize,
        actual: usize,
    },
    InvalidUser,
    InvalidCredentials,
    UserNotFound,
    UserAlreadyExists,
    JwtGenerationError,
    DatabaseError,
    InvalidToken,
}

impl UserErrorKind {
    pub fn message(&self) -> String {
        match self {
            UserErrorKind::EmptyName => "Empty name".to_string(),
            UserErrorKind::PasswordTooShort { required, actual } => format!("Password too short: {required} > {actual}"),
            UserErrorKind::InvalidUser => "Invalid user".to_string(),
            UserErrorKind::InvalidCredentials => "Invalid credentials".to_string(),
            UserErrorKind::UserNotFound => "User not found".to_string(),
            UserErrorKind::UserAlreadyExists => "User already exists".to_string(),
            UserErrorKind::JwtGenerationError => "JWT generation failed".to_string(),
            UserErrorKind::DatabaseError => "Unexpected database error".to_string(),
            UserErrorKind::InvalidToken => "Invalid token".to_string(),
        }
    }
}