use crate::application::user::requests::UserAuthenticateRequest;
use crate::domain::user::error::{UserError, UserErrorKind};
use crate::infrastructure::app_state::AppState;
use crate::infrastructure::error::HttpError;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;

/// Middleware that validates a User JWT, loads the [`User`](crate::domain::user::entities::User)
/// entity, and inserts it into request extensions.
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, HttpError> {
    // Extract the Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .map(String::from);

    if let Some(auth_header) = auth_header {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            let user = state
                .user_request_handler()
                .authenticate(UserAuthenticateRequest {
                    token: token.to_string(),
                })
                .await?;

            // Insert the authenticated user into request extensions
            req.extensions_mut().insert(user);

            return Ok(next.run(req).await);
        }
    }

    // If we reach here, authentication failed
    Err(UserError::new(UserErrorKind::InvalidCredentials).into())
}
