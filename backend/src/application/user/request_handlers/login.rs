//! # User Login Request Handler
//!
//! Handles user authentication via username and password.

use crate::application::user::contracts::{JwtGeneratorContract, UserRepositoryContract};
use crate::application::user::requests::UserLoginRequest;
use crate::application::user::responses::UserTokenResponse;
use crate::domain::user::entities::User;
use crate::domain::user::error::UserError;
use std::sync::Arc;

/// Handler for user login requests.
///
/// Validates user credentials and generates a JWT token upon successful authentication.
///
/// # Type Parameters
///
/// * `UserRepository` - Repository for user data access
/// * `JwtGenerator` - JWT token generator
pub struct UserLoginRequestHandler<UserRepository, JwtGenerator>
where
    UserRepository: UserRepositoryContract,
    JwtGenerator: JwtGeneratorContract,
{
    repository: Arc<UserRepository>,
    jwt: Arc<JwtGenerator>,
}

impl<UserRepository, JwtGenerator> UserLoginRequestHandler<UserRepository, JwtGenerator>
where
    UserRepository: UserRepositoryContract,
    JwtGenerator: JwtGeneratorContract,
{
    pub fn new(repository: Arc<UserRepository>, jwt: Arc<JwtGenerator>) -> Self {
        Self { repository, jwt }
    }

    pub async fn login(&self, request: UserLoginRequest) -> Result<UserTokenResponse, UserError> {
        let user: User = self.repository.get_by_name(&request.name).await?;

        user.check_password(request.password)?;

        let token = self.jwt.generate_token(user.id())?;
        Ok(UserTokenResponse { token })
    }
}

#[cfg(test)]
mod tests {
    use crate::application::user::contracts::{
        MockJwtGeneratorContract, MockUserRepositoryContract,
    };
    use crate::application::user::request_handlers::login::UserLoginRequestHandler;
    use crate::application::user::requests::UserLoginRequest;
    use crate::domain::common::identifier::Identifier;
    use crate::domain::user::entities::User;
    use crate::domain::user::error::{UserError, UserErrorKind};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_valid_login_returns_token() {
        let mut user_repo = MockUserRepositoryContract::new();
        let mut jwt_generator = MockJwtGeneratorContract::new();

        let name = "test-user".to_string();
        let password = "password".to_string();
        let request = UserLoginRequest {
            name: name.clone(),
            password: password.clone(),
        };

        let user_id = Identifier::new();
        let user = User::try_new(user_id.clone(), name.clone(), password.clone()).unwrap();
        user_repo.expect_get_by_name().times(1).returning(move |_| {
            let user = user.clone();
            Box::pin(async move { Ok(user) })
        });

        jwt_generator
            .expect_generate_token()
            .times(1)
            .returning(|_| Ok("login-token".to_string()));

        let handler = UserLoginRequestHandler::new(Arc::new(user_repo), Arc::new(jwt_generator));
        let result = handler.login(request).await;

        assert!(result.is_ok());
        let token_response = result.unwrap();
        assert_eq!(token_response.token, "login-token");
    }

    #[tokio::test]
    async fn test_invalid_password_login_returns_error() {
        let mut user_repo = MockUserRepositoryContract::new();
        let mut jwt_generator = MockJwtGeneratorContract::new();

        let name = "test-user".to_string();
        let password = "invalid-password".to_string();
        let request = UserLoginRequest {
            name: name.clone(),
            password: password.clone(),
        };

        let user_id = Identifier::new();
        let user =
            User::try_new(user_id.clone(), name.clone(), "real-password".to_string()).unwrap();
        user_repo.expect_get_by_name().times(1).returning(move |_| {
            let user = user.clone();
            Box::pin(async move { Ok(user) })
        });

        jwt_generator.expect_generate_token().never();

        let handler = UserLoginRequestHandler::new(Arc::new(user_repo), Arc::new(jwt_generator));
        let result = handler.login(request).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, UserError::new(UserErrorKind::InvalidCredentials));
    }
}
