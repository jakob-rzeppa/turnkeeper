use std::sync::LazyLock;
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Deserialize, Serialize};
use crate::error::HttpError;

const GM_PASSWORD: LazyLock<String> = LazyLock::new(|| {
    if cfg!(test) {
        return "test".to_string();
    }

    std::env::var("GM_PASSWORD")
        .expect("GM_PASSWORD environment variable is not set")
});

#[derive(Deserialize, JsonRequest, Debug)]
pub struct LoginRequest {
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
    Err(HttpError::NotImplemented)
}