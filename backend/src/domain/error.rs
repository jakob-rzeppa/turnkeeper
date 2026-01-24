#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    InvalidState {
        msg: String,
    },
    InvalidCredentials {
        msg: String,
    },
    Unauthorized {
        msg: String,
    },
    UnexpectedError {
        msg: String,
    },
    NotImplemented,
}

impl Error {
    /// Adds a prefix to the message
    ///
    /// If `prefix("test module")` is used on `DomainError::InvalidParameter { msg: "id can't be null" }`
    /// will return `DomainError::InvalidParameter { msg: "test module: id can't be null" }`.
    pub fn prefix(self, prefix: String) -> Self {
        match self {
            Error::InvalidState { msg } => Error::InvalidState { msg: format!("{0}: {1}", prefix, msg) },
            Error::InvalidCredentials { msg } => Error::InvalidCredentials { msg: format!("{0}: {1}", prefix, msg) },
            Error::Unauthorized { msg } => Error::Unauthorized { msg: format!("{0}: {1}", prefix, msg) },
            Error::UnexpectedError { msg } => Error::UnexpectedError { msg: format!("{0}: {1}", prefix, msg) },
            Error::NotImplemented => Error::NotImplemented,
        }
    }
}