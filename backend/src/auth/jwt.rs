use std::sync::LazyLock;
use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

const JWT_SECRET: LazyLock<String> = LazyLock::new(|| {
    if cfg!(test) {
        return "test secret".to_string();
    }

    std::env::var("JWT_SECRET")
        .expect("JWT_SECRET environment variable is not set")
});

// Define a struct to represent the claims in the JWT
#[derive(Serialize, Deserialize)]
struct Claims {
    user_id: u32,
    exp: usize,  // Expiration time
}

// Function to generate a JWT
fn generate_jwt(user_id: u32) -> String {
    let exp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() + 3600 * 5; // 5 hour expiration
    let claims = Claims { user_id, exp: exp as usize };

    let header = Header::new(Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(JWT_SECRET.as_bytes());

    encode(&header, &claims, &encoding_key).unwrap()
}

// Function to validate a JWT
fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(JWT_SECRET.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    decode::<Claims>(token, &decoding_key, &validation).map(|data| data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_and_validate_jwt() {
        let jwt = generate_jwt(1);

        assert!(validate_jwt(&jwt).is_ok());
    }

    #[test]
    fn invalid_jwt() {
        assert!(validate_jwt("aioefhaiöfake").is_err());
    }
}
