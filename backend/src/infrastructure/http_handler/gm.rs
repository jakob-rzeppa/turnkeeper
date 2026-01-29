use crate::infrastructure::error::HttpError;
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Deserialize, Serialize};

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
    // let gm_auth_handler = AuthenticateRequestHandler::new(JwtGenerator::new(), JwtValidator::new());
    //
    // let request_dto = LoginGmRequestDto { password: request.password };
    // let response = gm_auth_handler.login(request_dto)?;
    //
    // Ok(LoginResponse{ token: response.token })
    Err(HttpError::NotImplemented)
}
