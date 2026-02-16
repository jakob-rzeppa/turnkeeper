//! # User Contracts
//!
//! Defines traits (contracts) for user-related infrastructure dependencies.

use uuid::Uuid;
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
    async fn check_if_exists(&self, id: &Uuid) -> Result<bool, UserError>;

    /// Retrieves a user by their unique ID.
    ///
    /// # Errors
    ///
    /// Returns [`UserErrorKind::UserNotFound`] if no user exists with the given ID.
    async fn get_by_id(&self, id: &Uuid) -> Result<User, UserError>;

    /// Retrieves a user by their username.
    ///
    /// # Arguments
    ///
    /// * `name` - The username to search for
    ///
    /// # Errors
    ///
    /// Returns [`UserErrorKind::UserNotFound`] if no user exists with the given name.
    async fn get_by_name(&self, name: &str) -> Result<User, UserError>;

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
    async fn save(&self, user: &User) -> Result<(), UserError>;
}

/// Contract for generating JWT tokens for authenticated users.
///
/// Implementations should:
/// - Sign tokens with a secret key
/// - Include user ID in the token payload
/// - Set appropriate expiration times
///
/// # Examples
///
/// ```rust,ignore
/// let token = jwt_generator.generate_token(&user_id)?;
/// // Token can now be sent to the client
/// ```
#[mockall::automock]
pub trait UserJwtGeneratorContract {
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
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let token = generator.generate_token(&user.id())?;
    /// // Send token to client: {"token": "eyJ0eXAi..."}
    /// ```
    fn generate_token(&self, user_id: &Uuid) -> Result<String, UserError>;
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
///
/// # Examples
///
/// ```rust,ignore
/// // Typically called from middleware
/// let user_id = validator.validate_token("Bearer eyJ0eXAi...")?;
/// ```
#[mockall::automock]
pub trait UserJwtValidatorContract {
    /// Validates a JWT token and extracts the user ID.
    ///
    /// # Arguments
    ///
    /// * `bearer_token` - The token string, typically from the `Authorization` header. The "Bearer " should not be part of the token.
    ///
    /// # Returns
    ///
    /// * `Ok(Uuid)` - The user ID extracted from the valid token
    /// * `Err(UserError)` - Token is invalid, expired, or malformed
    ///
    /// # Errors
    ///
    /// Returns [`UserErrorKind::Unauthorized`] if:
    /// - Token signature is invalid
    /// - Token has expired
    /// - Token format is malformed
    /// - Bearer prefix is missing or incorrect
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Extract token from Authorization header
    /// let auth_header = "eyJ0eXAi...";
    /// let user_id = validator.validate_token(auth_header)?;
    /// ```
    fn validate_token(&self, token: &str) -> Result<Uuid, UserError>;
}