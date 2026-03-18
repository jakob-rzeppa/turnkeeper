//! # Game Master Contracts
//!
//! Defines traits (contracts) for Game Master authentication.
//!
//! GMs have elevated privileges and use a different authentication mechanism
//! than regular users (password from environment variable instead of database).

use crate::domain::gm::error::GmError;

/// Contract for generating JWT tokens for authenticated Game Masters.
///
/// Unlike user tokens, GM tokens:
/// - Don't contain a user ID (GMs are not users in the database)
/// - Are generated after validating against a master password
/// - Grant elevated privileges for game and user management
///
/// # Security
///
/// The GM authentication uses a single shared password stored in environment variables.
/// This is suitable for a trusted administrator in a private game system.
#[mockall::automock]
pub trait GmJwtGeneratorContract {
    /// Generates a JWT token for a Game Master.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - A signed JWT token as a string
    /// * `Err(GmError)` - Token generation failed
    fn generate_token(&self) -> Result<String, GmError>;
}

/// Contract for validating Game Master JWT tokens.
///
/// Implementations should:
/// - Verify token signature using the secret key
/// - Check token expiration
/// - Confirm the token has GM role claims
///
/// # Security
///
/// Validation ensures that:
/// - The token was issued by this server (signature check)
/// - The token hasn't expired
/// - The token hasn't been tampered with
/// - The token grants GM privileges
///
/// # Usage
///
/// This is typically called from middleware to protect GM-only endpoints.
#[mockall::automock]
pub trait GmJwtValidatorContract {
    /// Validates a GM JWT token.
    ///
    /// # Arguments
    ///
    /// * `token` - The token string, typically from the `Authorization` header
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Token is valid and grants GM privileges
    /// * `Err(GmError)` - Token is invalid, expired, or not a GM token
    fn validate_token(&self, token: &str) -> Result<(), GmError>;
}