use std::string::ToString;
use std::sync::LazyLock;
use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

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
pub fn generate_user_jwt(user_id: i64) -> Result<String, anyhow::Error> {
    let exp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600 * 5; // 5 hour expiration
    let claims = UserClaims { user_id, exp: exp as usize };

    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(USER_JWT_SECRET.as_bytes());

    Ok(encode(&header, &claims, &encoding_key)?)
}

pub fn generate_gm_jwt() -> Result<String, anyhow::Error> {
    let exp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 3600 * 5; // 5 hour expiration
    let claims = GmClaims { exp: exp as usize };

    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(GM_JWT_SECRET.as_bytes());

    Ok(encode(&header, &claims, &encoding_key)?)
}

// Functions to validate a JWT

/// returns the users id
pub fn validate_user_jwt(token: &str) -> Result<i64, anyhow::Error> {
    let decoding_key = DecodingKey::from_secret(USER_JWT_SECRET.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    let claims = decode::<UserClaims>(token, &decoding_key, &validation).map(|data| data.claims)?;

    Ok(claims.user_id)
}

pub fn validate_gm_jwt(token: &str) -> Result<(), anyhow::Error> {
    let decoding_key = DecodingKey::from_secret(GM_JWT_SECRET.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    let claims = decode::<GmClaims>(token, &decoding_key, &validation).map(|data| data.claims)?;

    Ok(())
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

        assert!(validate_gm_jwt(&jwt).is_err());
    }

    #[test]
    fn generate_gm_and_validate_user_jwt() {
        let jwt = generate_gm_jwt().unwrap();

        assert!(validate_user_jwt(&jwt).is_err());
    }

    #[test]
    fn invalid_gm_jwt() {
        assert!(validate_gm_jwt("aioefhaiöfake").is_err());
    }

    #[test]
    fn invalid_user_jwt() {
        assert!(validate_user_jwt("aioefhaiöfake").is_err());
    }
}
