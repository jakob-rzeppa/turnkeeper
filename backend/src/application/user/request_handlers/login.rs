//! # User Login Request Handler
//!
//! Handles user authentication via username and password.

use crate::application::user::contracts::{
    JwtGeneratorContract, JwtValidatorContract, UserRepositoryContract,
};
use crate::application::user::request_handlers::UserRequestHandler;
use crate::application::user::requests::UserLoginRequest;
use crate::application::user::responses::UserTokenResponse;
use crate::domain::user::entities::User;
use crate::domain::user::error::UserError;

impl<
    UserRepository: UserRepositoryContract,
    JwtGenerator: JwtGeneratorContract,
    JwtValidator: JwtValidatorContract,
> UserRequestHandler<UserRepository, JwtGenerator, JwtValidator>
{
    pub async fn login(&self, request: UserLoginRequest) -> Result<UserTokenResponse, UserError> {
        let user: User = self.user_repository.get_by_name(&request.name).await?;

        user.check_password(request.password)?;

        let token = self.jwt_generator.generate_token(user.id())?;
        Ok(UserTokenResponse { token })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        application::user::contracts::{
            MockJwtGeneratorContract, MockJwtValidatorContract, MockUserRepositoryContract,
        },
        domain::{common::identifier::Identifier, user::error::UserErrorKind},
    };

    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_valid_login_returns_token() {
        let mut user_repo = MockUserRepositoryContract::new();
        let mut jwt_generator = MockJwtGeneratorContract::new();
        let jwt_validator = MockJwtValidatorContract::new();

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

        let handler = UserRequestHandler::new(
            Arc::new(user_repo),
            Arc::new(jwt_generator),
            Arc::new(jwt_validator),
        );
        let result = handler.login(request).await;

        assert!(result.is_ok());
        let token_response = result.unwrap();
        assert_eq!(token_response.token, "login-token");
    }

    #[tokio::test]
    async fn test_invalid_password_login_returns_error() {
        let mut user_repo = MockUserRepositoryContract::new();
        let mut jwt_generator = MockJwtGeneratorContract::new();
        let jwt_validator = MockJwtValidatorContract::new();

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

        let handler = UserRequestHandler::new(
            Arc::new(user_repo),
            Arc::new(jwt_generator),
            Arc::new(jwt_validator),
        );
        let result = handler.login(request).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, UserError::new(UserErrorKind::InvalidCredentials));
    }
}
