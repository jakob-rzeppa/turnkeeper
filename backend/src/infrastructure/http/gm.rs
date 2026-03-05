use crate::infrastructure::error::HttpError;
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Deserialize, Serialize};
use crate::application::gm::request_handlers::login::GmLoginRequestHandler;
use crate::application::gm::requests::{GmLoginRequest};
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
