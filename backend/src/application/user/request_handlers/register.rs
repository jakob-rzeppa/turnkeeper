//! # User Registration Handler
//!
//! Creates a new user and returns a JWT token.

use crate::application::user::contracts::{
    JwtGeneratorContract, JwtValidatorContract, UserRepositoryContract,
};
use crate::application::user::request_handlers::UserRequestHandler;
use crate::application::user::requests::UserRegisterRequest;
use crate::application::user::responses::UserTokenResponse;
use crate::domain::common::identifier::Id;
use crate::domain::user::entities::User;
use crate::domain::user::error::UserError;

impl<
    UserRepository: UserRepositoryContract,
    JwtGenerator: JwtGeneratorContract,
    JwtValidator: JwtValidatorContract,
> UserRequestHandler<UserRepository, JwtGenerator, JwtValidator>
{
    /// Registers a new user and returns a JWT token.
    ///
    /// # Errors
    ///
    /// - [`UserErrorKind::EmptyName`] / [`UserErrorKind::PasswordTooShort`] — invalid input
    /// - [`UserErrorKind::UserAlreadyExists`] — duplicate username
    pub async fn register(
        &self,
        request: UserRegisterRequest,
    ) -> Result<UserTokenResponse, UserError> {
        let user = User::try_new(Id::new(), request.name, request.password)?;

        self.user_repository.save(&user).await?;

        let token = self.jwt_generator.generate_token(user.id())?;
        Ok(UserTokenResponse { token })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::application::user::contracts::{
        MockJwtGeneratorContract, MockJwtValidatorContract, MockUserRepositoryContract,
    };

    #[tokio::test]
    async fn test_valid_call_save_and_return_token() {
        let mut user_repo = MockUserRepositoryContract::new();
        let mut jwt_generator = MockJwtGeneratorContract::new();
        let jwt_validator = MockJwtValidatorContract::new();

        // Prepare test data
        let name = "test-user".to_string();
        let password = "password".to_string();
        let request = UserRegisterRequest {
            name: name.clone(),
            password: password.clone(),
        };

        // We don't care about the actual user, so use any()
        user_repo
            .expect_save()
            .times(1)
            .returning(|_| Box::pin(async move { Ok(()) }));

        // The token we expect to be returned
        jwt_generator
            .expect_generate_token()
            .times(1)
            .returning(|_| Ok("test-token".to_string()));

        let handler = UserRequestHandler::new(
            Arc::new(user_repo),
            Arc::new(jwt_generator),
            Arc::new(jwt_validator),
        );
        let result = handler.register(request).await;

        assert!(result.is_ok());
        let token_response = result.unwrap();
        assert_eq!(token_response.token, "test-token");
    }
}
