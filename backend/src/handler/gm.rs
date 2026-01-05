use crate::error::HttpError;
use crate::json_handler;

json_handler!(Login, {}, {
    token: String
});

/// POST /gm/login
///
/// authenticates the gm via a secret set in the environment variables
/// and returns a JSON WEB TOKEN
pub async fn login(request: LoginRequest) -> Result<LoginResponse, HttpError> {
    Err(HttpError::NotImplemented)
}