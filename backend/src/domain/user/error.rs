use std::fmt;

pub struct UserError {
    pub kind: UserErrorKind,
    pub source: Option<Box<UserError>>,
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
    JwtGenerationError(String),
    DatabaseError(String),
}

impl UserError {
    pub fn message(&self) -> String {
        match &self.kind {
            UserErrorKind::EmptyName => "Empty name".to_string(),
            UserErrorKind::PasswordTooShort { required, actual } => format!("Password too short: {required} > {actual}"),
            UserErrorKind::InvalidUser => "Invalid user".to_string(),
            UserErrorKind::InvalidCredentials => "Invalid credentials".to_string(),
            UserErrorKind::UserNotFound => "User not found".to_string(),
            UserErrorKind::UserAlreadyExists => "User already exists".to_string(),
            UserErrorKind::JwtGenerationError(e) => format!("JWT generation failed: {}", e),
            UserErrorKind::DatabaseError(e) => format!("Unexpected database error: {}", e),
        }
    }
}

impl PartialEq for UserError {
    // Only check the error kind, since errors are the same even if they have a different source
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl fmt::Debug for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UserError")
            .field("message", &self.message())
            .field("source", &self.source.as_ref().map(|e| e.to_string()))
            .finish()
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())?;
        let mut source = self.source.as_deref();
        while let Some(err) = source {
            write!(f, ": {}", err)?;
            source = err.source.as_deref();
        }
        Ok(())
    }
}

impl UserError {
    pub fn new(kind: UserErrorKind) -> Self {
        UserError { kind, source: None }
    }

    pub fn with_source(kind: UserErrorKind, source: UserError) -> Self
    {
        UserError {
            kind,
            source: Some(Box::new(source)),
        }
    }
}