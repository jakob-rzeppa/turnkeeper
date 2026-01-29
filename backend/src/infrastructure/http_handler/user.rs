use axum::extract::{State};
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Serialize, Deserialize};
use crate::{AppState};
use crate::application::user::request_handlers::login::LoginRequestHandler;
use crate::application::user::request_handlers::register::RegisterRequestHandler;
use crate::application::user::requests::{UserLoginRequest, UserRegisterRequest};
use crate::infrastructure::auth::jwt::{JwtGenerator, JwtValidator};
use crate::infrastructure::error::HttpError;
use crate::infrastructure::repository::user::SqliteUserRepository;

#[derive(Deserialize, JsonRequest, Debug)]
pub struct LoginHttpRequest {
    name: String,
    password: String
}

#[derive(Serialize, JsonResponse, Debug)]
pub struct LoginHttpResponse {
    token: String,
}

/// POST /user/login
///
/// authenticates a user via username and password and returns a JSON WEB TOKEN
pub async fn login(State(state): State<AppState>, payload: LoginHttpRequest) -> Result<LoginHttpResponse, HttpError> {
    let user_auth_handler = LoginRequestHandler::new(
        SqliteUserRepository::new(state.db.clone()),
        JwtGenerator::new(),
    );

    let request_dto = UserLoginRequest { name: payload.name, password: payload.password };
    let result = user_auth_handler.login(request_dto).await?;

    Ok(LoginHttpResponse {
        token: result.token
    })
}

#[derive(Deserialize, JsonRequest)]
pub struct RegisterHttpRequest {
    name: String,
    password: String
}

#[derive(Serialize, JsonResponse)]
pub struct RegisterHttpResponse {
    token: String,
}

/// POST /user/register
///
/// registers a new user via username and password
pub async fn register(State(state): State<AppState>, payload: RegisterHttpRequest) -> Result<RegisterHttpResponse, HttpError> {
    let user_auth_handler = RegisterRequestHandler::new(
        SqliteUserRepository::new(state.db.clone()),
        JwtGenerator::new(),
    );

    let request_dto = UserRegisterRequest { name: payload.name, password: payload.password };
    let result = user_auth_handler.register(request_dto).await?;

    Ok(RegisterHttpResponse {
        token: result.token
    })
}
