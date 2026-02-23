use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{Response};
use crate::application::gm::contracts::GmJwtValidatorContract;
use crate::AppState;
use crate::domain::gm::error::{GmError, GmErrorKind};
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