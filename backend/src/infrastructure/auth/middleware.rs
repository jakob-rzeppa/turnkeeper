use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use crate::application::gm::contracts::GmJwtValidatorContract;
use crate::application::user::contracts::{UserJwtValidatorContract, UserRepositoryContract};
use crate::AppState;
use crate::domain::gm::error::{GmError, GmErrorKind};
use crate::domain::user::error::{UserError, UserErrorKind};
use crate::infrastructure::error::HttpError;

pub async fn gm_auth_middleware(State(state): State<AppState>, req: Request, next: Next) -> Result<Response, HttpError> {
    // Extract the Authorization header
    let auth_header = req.headers().get("Authorization").and_then(|h| h.to_str().ok());

    // Check if the header is present and starts with "Bearer "
    if let Some(auth_header) = auth_header {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            if state.auth_manager.gm_jwt_validator().validate_token(token).is_ok() {
                // Token is valid, proceed to the next middleware or handler
                return Ok(next.run(req).await);
            }
        }
    }

    // If we reach here, authentication failed
    Err(GmError::new(GmErrorKind::Unauthorized).into())
}

pub async fn user_auth_middleware(State(state): State<AppState>, mut req: Request, next: Next) -> Result<Response, HttpError> {
    // Extract the Authorization header
    let auth_header = req.headers().get("Authorization").and_then(|h| h.to_str().ok()).map(String::from);

    if let Some(auth_header) = auth_header {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            let user_id = state
                .auth_manager
                .user_jwt_validator()
                .validate_token(token)
                .map_err(|_| HttpError::from(UserError::new(UserErrorKind::InvalidToken)))?;

            let user = state.repository_manager.user().get_by_id(&user_id).await?;

            // Insert the authenticated user into request extensions
            req.extensions_mut().insert(user);

            return Ok(next.run(req).await);
        }
    }

    // If we reach here, authentication failed
    Err(UserError::new(UserErrorKind::InvalidCredentials).into())
}