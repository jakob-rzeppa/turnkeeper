use axum::extract::{State};
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Serialize, Deserialize};
use crate::{AppState};
use crate::application::auth::dto::{LoginUserRequestDto, RegisterUserRequestDto};
use crate::application::auth::user_handler::UserAuthHandler;
use crate::infrastructure::auth::jwt::{JwtGenerator, JwtValidator};
use crate::infrastructure::error::HttpError;
use crate::infrastructure::repository::user::SqliteUserRepository;

#[derive(Deserialize, JsonRequest, Debug)]
pub struct LoginRequest {
    name: String,
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
    let user_auth_handler = UserAuthHandler::new(
        SqliteUserRepository::new(state.db.clone()),
        JwtGenerator::new(),
        JwtValidator::new(),
    );

    let request_dto = LoginUserRequestDto { name: payload.name, password: payload.password };
    let result = user_auth_handler.login(request_dto).await?;

    Ok(LoginResponse {
        token: result.token
    })
}

#[derive(Deserialize, JsonRequest)]
pub struct RegisterRequest {
    name: String,
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
    let user_auth_handler = UserAuthHandler::new(
        SqliteUserRepository::new(state.db.clone()),
        JwtGenerator::new(),
        JwtValidator::new(),
    );

    let request_dto = RegisterUserRequestDto { name: payload.name, password: payload.password };
    let result = user_auth_handler.register(request_dto).await?;

    Ok(RegisterResponse {
        token: result.token
    })
}
