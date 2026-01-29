use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::application::gm::contracts::{GmJwtGeneratorContract, GmJwtValidatorContract};
use crate::application::user::contracts::{UserJwtGeneratorContract, UserJwtValidatorContract};
use crate::domain::error::Error;

const USER_JWT_SECRET: LazyLock<String> = LazyLock::new(|| {
    if cfg!(test) {
        return "user test secret".to_string();
    }

    std::env::var("USER_JWT_SECRET")
        .expect("JWT_SECRET environment variable is not set")
});

#[derive(Serialize, Deserialize)]
struct UserClaims {
    user_id: String,
    exp: usize,  // Expiration time
}

impl From<Uuid> for UserClaims {
    fn from(id: Uuid) -> Self {
        let exp = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() + 3600 * 5; // 5 hour expiration

        UserClaims { user_id: id.to_string(), exp: exp as usize }
    }
}

pub struct UserJwtGenerator {}

impl UserJwtGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

impl UserJwtGeneratorContract for UserJwtGenerator {
    fn generate_token(&self, user_id: &Uuid) -> Result<String, Error> {
        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(USER_JWT_SECRET.as_bytes());

        let claims = UserClaims::from(user_id.clone());

        Ok(encode(&header, &claims, &encoding_key)
            .map_err(|e| Error::UnexpectedError { msg: e.to_string() })?)
    }
}

pub struct UserJwtValidator {}

impl UserJwtValidator {
    pub fn new() -> Self {
        Self {}
    }
}

impl UserJwtValidatorContract for UserJwtValidator {
    fn validate_token(&self, token: &str) -> Result<Uuid, Error> {
        let decoding_key = DecodingKey::from_secret(USER_JWT_SECRET.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        let claims = decode::<UserClaims>(token, &decoding_key, &validation).map(|data| data.claims)
            .map_err(|e| Error::InvalidCredentials { msg: e.to_string() })?;

        Ok(Uuid::try_from(claims.user_id).map_err(|_| Error::InvalidCredentials { msg: "Invalid user token: Invalid uuid".to_string() })?)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use super::*;

    const JWT_GENERATOR: UserJwtGenerator = UserJwtGenerator {};
    const JWT_VALIDATOR: UserJwtValidator = UserJwtValidator {};

    #[test]
    fn generate_and_validate_jwt() {
        let id = Uuid::new_v4();

        let token = JWT_GENERATOR.generate_token(&id).unwrap();

        let user_id = JWT_VALIDATOR.validate_token(&token).unwrap();

        assert_eq!(user_id, id);
    }

    #[test]
    fn invalid_gm_jwt() {
        let token = "invalid".to_string();

        assert!(matches!(JWT_VALIDATOR.validate_token(&token), Err(Error::InvalidCredentials { .. })));
    }
}