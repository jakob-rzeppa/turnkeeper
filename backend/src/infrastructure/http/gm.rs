use axum::extract::State;
use crate::infrastructure::error::HttpError;
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Deserialize, Serialize};
use crate::application::gm::request_handlers::login::GmLoginRequestHandler;
use crate::application::gm::requests::{GmLoginRequest};
use crate::application::user::request_handlers::user_list::UserListRequestHandler;
use crate::AppState;
use crate::infrastructure::auth::gm_jwt::GmJwtGenerator;

#[derive(Deserialize, JsonRequest, Debug)]
pub struct LoginRequest {
    pub password: String,
}

#[derive(Serialize, JsonResponse, Debug)]
pub struct LoginResponse {
    pub token: String,
}

/// POST /gm/login
///
/// authenticates the gm via a secret set in the environment variables
/// and returns a JSON WEB TOKEN
pub async fn login(request: LoginRequest) -> Result<LoginResponse, HttpError> {
    let gm_auth_handler = GmLoginRequestHandler::new(GmJwtGenerator::new());

    let request = GmLoginRequest {
        password: request.password,
    };
    let result = gm_auth_handler.login(request).await?;

    Ok(LoginResponse {
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

/// GET /gm/users and /user/users
///
/// returns a list of all registered users
pub async fn list(State(state): State<AppState>) -> Result<UserListHttpResponse, HttpError> {
    let user_list_handler = UserListRequestHandler::new(
        state.repository_manager.user(),
    );

    let result = user_list_handler.list().await?;

    Ok(UserListHttpResponse {
        users: result.into_iter().map(|user| UserListHttpResponseUserListProjection {
            id: user.id.to_string(),
            name: user.name,
        }).collect()
    })
}