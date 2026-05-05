use crate::application::user::contracts::{JwtGeneratorContract, JwtValidatorContract};
use crate::domain::common::identifier::Id;
use crate::domain::user::error::{UserError, UserErrorKind};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};

const USER_JWT_SECRET: LazyLock<String> = LazyLock::new(|| {
    if cfg!(test) {
        return "user test secret".to_string();
    }

    std::env::var("USER_JWT_SECRET").expect("JWT_SECRET environment variable is not set")
});

#[derive(Serialize, Deserialize)]
struct UserClaims {
    user_id: String,
    exp: usize, // Expiration time
}

impl From<Id> for UserClaims {
    fn from(id: Id) -> Self {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            + 3600 * 5; // 5 hour expiration

        UserClaims {
            user_id: id.to_string(),
            exp: exp as usize,
        }
    }
}

pub struct JwtGenerator {}

impl JwtGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

impl JwtGeneratorContract for JwtGenerator {
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
    fn generate_token(&self, user_id: &Id) -> Result<String, UserError> {
        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(USER_JWT_SECRET.as_bytes());

        let claims = UserClaims::from(user_id.clone());

        Ok(encode(&header, &claims, &encoding_key)
            .map_err(|e| UserError::with_source(UserErrorKind::JwtGenerationError, Box::new(e)))?)
    }
}

pub struct JwtValidator {}

impl JwtValidator {
    pub fn new() -> Self {
        Self {}
    }
}

impl JwtValidatorContract for JwtValidator {
    /// Validates a JWT token and extracts the user ID.
    ///
    /// # Arguments
    ///
    /// * `bearer_token` - The token string, typically from the `Authorization` header. The "Bearer " should not be part of the token.
    ///
    /// # Returns
    ///
    /// * `Ok(Id)` - The user ID extracted from the valid token
    /// * `Err(UserError)` - Token is invalid, expired, or malformed
    ///
    /// # Errors
    ///
    /// Returns [`UserErrorKind::InvalidToken`] if:
    /// - Token signature is invalid
    /// - Token has expired
    /// - Token format is malformed
    fn validate_token(&self, token: &str) -> Result<Id, UserError> {
        let decoding_key = DecodingKey::from_secret(USER_JWT_SECRET.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        let claims = decode::<UserClaims>(token, &decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|_| UserError::new(UserErrorKind::InvalidCredentials))?;

        Ok(Id::parse_str(&claims.user_id)
            .map_err(|_| UserError::new(UserErrorKind::InvalidCredentials))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const JWT_GENERATOR: JwtGenerator = JwtGenerator {};
    const JWT_VALIDATOR: JwtValidator = JwtValidator {};

    #[test]
    fn generate_and_validate_jwt() {
        let id = Id::new();

        let token = JWT_GENERATOR.generate_token(&id).unwrap();

        let user_id = JWT_VALIDATOR.validate_token(&token).unwrap();

        assert_eq!(user_id, id);
    }

    #[test]
    fn invalid_jwt() {
        let token = "invalid".to_string();

        let result = JWT_VALIDATOR.validate_token(&token);

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, UserError::new(UserErrorKind::InvalidCredentials));
    }
}
