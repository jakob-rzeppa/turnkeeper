//! # User Error
//!
//! Error types for user domain operations.

use std::cmp::PartialEq;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Domain error for user-related operations.
///
/// Wraps a [`UserErrorKind`] discriminant and an optional source error.
#[derive(Debug)]
pub struct UserError {
    pub kind: UserErrorKind,
    source: Option<Box<dyn Error + 'static>>,
}

impl UserError {
    /// Creates a new `UserError` with the given kind and no source.
    pub fn new(kind: UserErrorKind) -> Self {
        UserError { kind, source: None }
    }
    /// Creates a new `UserError` with a source error for chaining.
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

/// Discriminant for [`UserError`], indicating the category of failure.
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
    /// Returns a human-readable message for this error kind.
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