use crate::application::user::requests::{UserLoginRequest, UserRegisterRequest};
use crate::infrastructure::app_state::AppState;
use crate::infrastructure::error::HttpError;
use axum::extract::State;
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, JsonRequest, Debug)]
pub struct LoginHttpRequest {
    name: String,
    password: String,
}

#[derive(Serialize, JsonResponse, Debug)]
pub struct LoginHttpResponse {
    token: String,
}

/// POST /login
///
/// authenticates a user via username and password and returns a JSON WEB TOKEN
pub async fn login(
    State(state): State<AppState>,
    payload: LoginHttpRequest,
) -> Result<LoginHttpResponse, HttpError> {
    let request_dto = UserLoginRequest {
        name: payload.name,
        password: payload.password,
    };
    let result = state.user_request_handler().login(request_dto).await?;

    Ok(LoginHttpResponse {
        token: result.token,
    })
}

#[derive(Deserialize, JsonRequest)]
pub struct RegisterHttpRequest {
    name: String,
    password: String,
}

#[derive(Serialize, JsonResponse)]
pub struct RegisterHttpResponse {
    token: String,
}

/// POST /register
///
/// registers a new user via username and password
pub async fn register(
    State(state): State<AppState>,
    payload: RegisterHttpRequest,
) -> Result<RegisterHttpResponse, HttpError> {
    let request_dto = UserRegisterRequest {
        name: payload.name,
        password: payload.password,
    };
    let result = state.user_request_handler().register(request_dto).await?;

    Ok(RegisterHttpResponse {
        token: result.token,
    })
}

#[derive(Serialize)]
pub struct UserListHttpResponseUserListProjection {
    id: String,
    name: String,
}

#[derive(Serialize, JsonResponse)]
pub struct UserListHttpResponse {
    users: Vec<UserListHttpResponseUserListProjection>,
}

/// GET /users
///
/// returns a list of all registered users
pub async fn list(State(state): State<AppState>) -> Result<UserListHttpResponse, HttpError> {
    let result = state.user_request_handler().list().await?;

    Ok(UserListHttpResponse {
        users: result
            .into_iter()
            .map(|user| UserListHttpResponseUserListProjection {
                id: user.id.to_string(),
                name: user.name,
            })
            .collect(),
    })
}
