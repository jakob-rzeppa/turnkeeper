use std::sync::LazyLock;
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use crate::infrastructure::auth::jwt::generate_gm_jwt;
use crate::error::HttpError;

const GM_PASSWORD: LazyLock<String> = LazyLock::new(|| {
    if cfg!(test) {
        return "test".to_string();
    }

    std::env::var("GM_PASSWORD")
        .expect("GM_PASSWORD environment variable is not set")
});

#[derive(Deserialize, Validate, JsonRequest, Debug)]
pub struct LoginRequest {
    #[validate(min_length = 1)]
    password: String,
}

#[derive(Serialize, JsonResponse, Debug)]
pub struct LoginResponse {
    token: String,
}
/// POST /gm/login
///
/// authenticates the gm via a secret set in the environment variables
/// and returns a JSON WEB TOKEN
pub async fn login(request: LoginRequest) -> Result<LoginResponse, HttpError> {
    if request.password != GM_PASSWORD.to_string() {
        return Err(HttpError::Unauthorized("Invalid credentials".to_string()));
    }

    let token = generate_gm_jwt().map_err(|e| e.into())?;

    Ok(LoginResponse { token })
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::auth::jwt::generate_gm_jwt_mock;
    use super::*;

    #[tokio::test]
    async fn test_returns_token() {
        generate_gm_jwt_mock::setup(|()| {
            Ok("test token".to_string())
        });

        let payload = LoginRequest { password: "test".to_string() };

        let result = login(payload).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.token, "test token".to_string());

        generate_gm_jwt_mock::assert_times(1);
    }

    #[tokio::test]
    async fn test_returns_no_token_if_invalid() {
        let payload = LoginRequest { password: "invalid".to_string() };

        let result = login(payload).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, HttpError::Unauthorized("Invalid credentials".to_string()));

        generate_gm_jwt_mock::assert_times(0);
    }
}