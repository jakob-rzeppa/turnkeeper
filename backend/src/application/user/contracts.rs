//! # User Contracts
//!
//! Defines traits (contracts) for user-related infrastructure dependencies.

use crate::domain::common::identifier::Identifier;
use crate::domain::user::entities::User;
use crate::domain::user::error::UserError;

/// Repository contract for user data persistence.
///
/// Defines the interface for user data access operations. Concrete implementations
/// are provided by the infrastructure layer (e.g., [`SqliteUserRepository`]).
#[mockall::automock]
pub trait UserRepositoryContract {
    /// Checks if a user exists by ID.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - User exists
    /// * `Ok(false)` - User does not exist
    /// * `Err(UserError)` - Database error occurred
    fn check_if_exists(
        &self,
        id: &Identifier,
    ) -> impl Future<Output = Result<bool, UserError>> + Send;

    /// Retrieves a user by their unique ID.
    ///
    /// # Errors
    ///
    /// Returns [`UserErrorKind::UserNotFound`] if no user exists with the given ID.
    fn get_by_id(&self, id: &Identifier) -> impl Future<Output = Result<User, UserError>> + Send;

    /// Retrieves a user by their username.
    ///
    /// # Arguments
    ///
    /// * `name` - The username to search for
    ///
    /// # Errors
    ///
    /// Returns [`UserErrorKind::UserNotFound`] if no user exists with the given name.
    fn get_by_name(&self, name: &str) -> impl Future<Output = Result<User, UserError>> + Send;

    /// Retrieves all users from the database.
    fn get_all(&self) -> impl Future<Output = Result<Vec<User>, UserError>> + Send;

    /// Persists a new user to the database.
    ///
    /// # Errors
    ///
    /// Returns [`UserErrorKind::UserAlreadyExists`] if:
    /// - A user with the same ID already exists
    /// - A user with the same name already exists (names must be unique)
    ///
    /// # Implementation Note
    ///
    /// Implementations must enforce username uniqueness.
    fn save(&self, user: &User) -> impl Future<Output = Result<(), UserError>> + Send;
}

/// Contract for generating JWT tokens for authenticated users.
///
/// Implementations should:
/// - Sign tokens with a secret key
/// - Include user ID in the token payload
/// - Set appropriate expiration times
#[mockall::automock]
pub trait JwtGeneratorContract {
    /// Generates a JWT token for a user.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - A signed JWT token as a string
    /// * `Err(UserError)` - Token generation failed
    ///
    /// # Token Format
    ///
    /// The token includes:
    /// - User ID in the claims
    /// - Expiration timestamp
    /// - Signature using the secret key
    fn generate_token(&self, user_id: &Identifier) -> Result<String, UserError>;
}

/// Contract for validating JWT tokens and extracting user information.
///
/// Implementations should:
/// - Verify token signature using the secret key
/// - Check token expiration
/// - Extract and return the user ID from valid tokens
///
/// # Security
///
/// Validation ensures that:
/// - The token was issued by this server (signature check)
/// - The token hasn't expired
/// - The token hasn't been tampered with
#[mockall::automock]
pub trait JwtValidatorContract {
    /// Validates a JWT token and extracts the user ID.
    ///
    /// # Arguments
    ///
    /// * `bearer_token` - The token string, typically from the `Authorization` header. The "Bearer " should not be part of the token.
    ///
    /// # Returns
    ///
    /// * `Ok(Identifier)` - The user ID extracted from the valid token
    /// * `Err(UserError)` - Token is invalid, expired, or malformed
    ///
    /// # Errors
    ///
    /// Returns [`UserErrorKind::InvalidToken`] if:
    /// - Token signature is invalid
    /// - Token has expired
    /// - Token format is malformed
    fn validate_token(&self, token: &str) -> Result<Identifier, UserError>;
}
