use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::application::gm::contracts::{GmJwtGeneratorContract, GmJwtValidatorContract};
use crate::domain::error::Error;

const GM_JWT_SECRET: LazyLock<String> = LazyLock::new(|| {
    if cfg!(test) {
        return "gm test secret".to_string();
    }

    std::env::var("GM_JWT_SECRET")
        .expect("JWT_SECRET environment variable is not set")
});

#[derive(Serialize, Deserialize)]
struct GmClaims {
    exp: usize,  // Expiration time
}

impl GmClaims {
    fn new() -> Self {
        let exp = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() + 3600 * 5; // 5 hour expiration

        Self { exp: exp as usize }
    }
}

pub struct GmJwtGenerator {}

impl GmJwtGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

impl GmJwtGeneratorContract for GmJwtGenerator {
    fn generate_token(&self) -> Result<String, Error> {
        let exp = SystemTime::now().duration_since(UNIX_EPOCH)
            .map_err(|e| Error::UnexpectedError { msg: e.to_string() })?
            .as_secs() + 3600 * 5; // 5 hour expiration
        let claims = GmClaims { exp: exp as usize };

        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(GM_JWT_SECRET.as_bytes());

        Ok(encode(&header, &claims, &encoding_key)
            .map_err(|e| Error::UnexpectedError { msg: e.to_string() })?)
    }
}

pub struct GmJwtValidator {}

impl GmJwtValidator {
    pub fn new() -> Self {
        Self {}
    }
}

impl GmJwtValidatorContract for GmJwtValidator {
    fn validate_token(&self, token: &str) -> Result<(), Error> {
        let decoding_key = DecodingKey::from_secret(GM_JWT_SECRET.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        decode::<GmClaims>(token, &decoding_key, &validation).map(|data| data.claims)
            .map_err(|e| Error::InvalidCredentials { msg: e.to_string() })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use super::*;

    const JWT_GENERATOR: GmJwtGenerator = GmJwtGenerator {};
    const JWT_VALIDATOR: GmJwtValidator = GmJwtValidator {};

    #[test]
    fn generate_and_validate_jwt() {
        let token = JWT_GENERATOR.generate_token().unwrap();

        assert!(JWT_VALIDATOR.validate_token(&token).is_ok());
    }

    #[test]
    fn invalid_gm_jwt() {
        let token = "invalid".to_string();

        assert!(matches!(JWT_VALIDATOR.validate_token(&token), Err(Error::InvalidCredentials { .. })));
    }
}