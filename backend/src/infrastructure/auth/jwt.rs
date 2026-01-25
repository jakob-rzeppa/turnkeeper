use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, decode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::application::auth::dto::BearerToken;
use crate::domain::auth::jwt::{JwtGeneratorTrait, JwtValidatorTrait};
use crate::domain::error::Error;

const GM_JWT_SECRET: LazyLock<String> = LazyLock::new(|| {
    if cfg!(test) {
        return "gm test secret".to_string();
    }

    std::env::var("GM_JWT_SECRET")
        .expect("JWT_SECRET environment variable is not set")
});

const USER_JWT_SECRET: LazyLock<String> = LazyLock::new(|| {
    if cfg!(test) {
        return "user test secret".to_string();
    }

    std::env::var("USER_JWT_SECRET")
        .expect("JWT_SECRET environment variable is not set")
});

// Define structs to represent the claims in the JWT
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

pub struct JwtGenerator {}

impl JwtGeneratorTrait for JwtGenerator {
    fn generate_user_token(&self, user_id: Uuid) -> Result<String, Error> {
        let header = Header::new(Algorithm::HS256);
        let encoding_key = EncodingKey::from_secret(USER_JWT_SECRET.as_bytes());

        let claims = UserClaims::from(user_id);

        Ok(encode(&header, &claims, &encoding_key)
            .map_err(|e| Error::UnexpectedError { msg: e.to_string() })?)
    }

    fn generate_gm_token(&self) -> Result<String, Error> {
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

pub struct JwtValidator {}

impl JwtValidatorTrait for JwtValidator {
    fn validate_user_token(&self, bearer_token: BearerToken) -> Result<Uuid, Error> {
        let decoding_key = DecodingKey::from_secret(USER_JWT_SECRET.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        let token = bearer_token.token.as_str();

        let claims = decode::<UserClaims>(token, &decoding_key, &validation).map(|data| data.claims)
            .map_err(|e| Error::InvalidCredentials { msg: e.to_string() })?;

        Ok(Uuid::try_from(claims.user_id).map_err(|_| Error::InvalidCredentials { msg: "Invalid user token: Invalid uuid".to_string() })?)
    }

    fn validate_gm_token(&self, bearer_token: BearerToken) -> Result<(), Error> {
        let decoding_key = DecodingKey::from_secret(GM_JWT_SECRET.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        let token = bearer_token.token.as_str();

        decode::<GmClaims>(token, &decoding_key, &validation).map(|data| data.claims)
            .map_err(|e| Error::InvalidCredentials { msg: e.to_string() })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use crate::application::auth::dto::BearerToken;
    use super::*;

    const JWT_GENERATOR: JwtGenerator = JwtGenerator {};
    const JWT_VALIDATOR: JwtValidator = JwtValidator {};

    #[test]
    fn generate_and_validate_user_jwt() {
        let id = Uuid::new_v4();

        let jwt = JWT_GENERATOR.generate_user_token(id.clone()).unwrap();

        let bearer_token = BearerToken::new(jwt);

        let user_id = JWT_VALIDATOR.validate_user_token(bearer_token).unwrap();

        assert_eq!(user_id, id);
    }

    #[test]
    fn generate_and_validate_gm_jwt() {
        let jwt = JWT_GENERATOR.generate_gm_token().unwrap();

        let bearer_token = BearerToken::new(jwt);

        assert!(JWT_VALIDATOR.validate_gm_token(bearer_token).is_ok());
    }

    #[test]
    fn generate_user_and_validate_gm_jwt() {
        let id = Uuid::new_v4();

        let jwt = JWT_GENERATOR.generate_user_token(id).unwrap();

        let bearer_token = BearerToken::new(jwt);

        assert_eq!(JWT_VALIDATOR.validate_gm_token(bearer_token), Err(Error::InvalidCredentials {
            msg: "InvalidSignature".to_string(),
        }));
    }

    #[test]
    fn generate_gm_and_validate_user_jwt() {
        let jwt = JWT_GENERATOR.generate_gm_token().unwrap();

        let bearer_token = BearerToken::new(jwt);

        assert!(matches!(JWT_VALIDATOR.validate_user_token(bearer_token), Err(Error::InvalidCredentials { .. })));
    }

    #[test]
    fn invalid_gm_jwt() {
        let bearer_token = BearerToken::new("invalid".to_string());

        assert!(matches!(JWT_VALIDATOR.validate_gm_token(bearer_token), Err(Error::InvalidCredentials { .. })));
    }

    #[test]
    fn invalid_user_jwt() {
        let bearer_token = BearerToken::new("invalid".to_string());

        assert!(matches!(JWT_VALIDATOR.validate_user_token(bearer_token), Err(Error::InvalidCredentials { .. })));
    }
}