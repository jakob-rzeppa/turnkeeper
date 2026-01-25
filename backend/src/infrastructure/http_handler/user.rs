use axum::extract::{State};
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Serialize, Deserialize};
use serde_valid::Validate;
use crate::{AppState};
use crate::infrastructure::error::HttpError;

#[derive(Deserialize, Validate, JsonRequest, Debug)]
pub struct LoginRequest {
    #[validate(min_length = 1)]
    name: String,
    #[validate(min_length = 1)]
    password: String
}

#[derive(Serialize, JsonResponse, Debug)]
pub struct LoginResponse {
    token: String,
}

/// POST /user/login
///
/// authenticates a user via username and password and returns a JSON WEB TOKEN
pub async fn login(State(state): State<AppState>, payload: LoginRequest) -> Result<LoginResponse, HttpError> {
    Err(HttpError::NotImplemented)
}

#[derive(Deserialize, Validate, JsonRequest)]
pub struct RegisterRequest {
    #[validate(min_length = 1)]
    name: String,
    #[validate(min_length = 1)]
    password: String
}

#[derive(Serialize, JsonResponse)]
pub struct RegisterResponse {
    token: String,
}

/// POST /user/register
///
/// registers a new user via username and password
pub async fn register(State(state): State<AppState>, payload: RegisterRequest) -> Result<RegisterResponse, HttpError> {
    Err(HttpError::NotImplemented)
}
