use std::string::ToString;
use std::sync::LazyLock;
use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::error::JwtError;

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
    user_id: i64,
    exp: usize,  // Expiration time
}
#[derive(Serialize, Deserialize)]
struct GmClaims {
    exp: usize,  // Expiration time
}


// Functions to generate a JWT
pub fn generate_user_jwt(user_id: i64) -> Result<String, JwtError> {
    let exp = SystemTime::now().duration_since(UNIX_EPOCH)
        .map_err(|e| JwtError::TimeError(e.to_string()))?
        .as_secs() + 3600 * 5; // 5 hour expiration
    let claims = UserClaims { user_id, exp: exp as usize };

    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(USER_JWT_SECRET.as_bytes());

    Ok(encode(&header, &claims, &encoding_key)
        .map_err(|e| JwtError::EncodeError(e.to_string()))?)
}

pub fn generate_gm_jwt() -> Result<String, JwtError> {
    let exp = SystemTime::now().duration_since(UNIX_EPOCH)
        .map_err(|e| JwtError::TimeError(e.to_string()))?
        .as_secs() + 3600 * 5; // 5 hour expiration
    let claims = GmClaims { exp: exp as usize };

    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(GM_JWT_SECRET.as_bytes());

    Ok(encode(&header, &claims, &encoding_key)
        .map_err(|e| JwtError::EncodeError(e.to_string()))?)
}

// Functions to validate a JWT

/// returns the users id
pub fn validate_user_jwt(token: &str) -> Result<i64, JwtError> {
    let decoding_key = DecodingKey::from_secret(USER_JWT_SECRET.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    let claims = decode::<UserClaims>(token, &decoding_key, &validation).map(|data| data.claims)
        .map_err(|e| JwtError::DecodeError(e.to_string()))?;

    Ok(claims.user_id)
}

pub fn validate_gm_jwt(token: &str) -> Result<(), JwtError> {
    let decoding_key = DecodingKey::from_secret(GM_JWT_SECRET.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    decode::<GmClaims>(token, &decoding_key, &validation)
        .map(|_| ())
        .map_err(|e| JwtError::DecodeError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_and_validate_user_jwt() {
        let jwt = generate_user_jwt(1).unwrap();

        let user_id = validate_user_jwt(&jwt).unwrap();

        assert_eq!(user_id, 1);
    }

    #[test]
    fn generate_and_validate_gm_jwt() {
        let jwt = generate_gm_jwt().unwrap();

        assert!(validate_gm_jwt(&jwt).is_ok());
    }

    #[test]
    fn generate_user_and_validate_gm_jwt() {
        let jwt = generate_user_jwt(1).unwrap();

        assert!(matches!(validate_gm_jwt(&jwt), Err(JwtError::DecodeError(_))));
    }

    #[test]
    fn generate_gm_and_validate_user_jwt() {
        let jwt = generate_gm_jwt().unwrap();

        assert!(matches!(validate_user_jwt(&jwt), Err(JwtError::DecodeError(_))));
    }

    #[test]
    fn invalid_gm_jwt() {
        assert!(matches!(validate_gm_jwt("aioefhaiöfake"), Err(JwtError::DecodeError(_))));
    }

    #[test]
    fn invalid_user_jwt() {
        assert!(matches!(validate_user_jwt("aioefhaiöfake"), Err(JwtError::DecodeError(_))));
    }
}
