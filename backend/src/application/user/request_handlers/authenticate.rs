//! # User Authentication Handler
//!
//! Validates a user JWT token and confirms the user exists.

use crate::application::user::contracts::{
    JwtGeneratorContract, JwtValidatorContract, UserRepositoryContract,
};
use crate::application::user::request_handlers::UserRequestHandler;
use crate::application::user::requests::UserAuthenticateRequest;
use crate::domain::user::entities::User;
use crate::domain::user::error::{UserError, UserErrorKind};

impl<
    UserRepository: UserRepositoryContract,
    JwtGenerator: JwtGeneratorContract,
    JwtValidator: JwtValidatorContract,
> UserRequestHandler<UserRepository, JwtGenerator, JwtValidator>
{
    /// Validates a JWT and checks that the referenced user still exists.
    pub async fn authenticate(&self, request: UserAuthenticateRequest) -> Result<User, UserError> {
        let user_id = self.jwt_validator.validate_token(&request.token)?;

        if !self.user_repository.check_if_exists(&user_id).await? {
            return Err(UserError::new(UserErrorKind::UserNotFound));
        }

        let user = self.user_repository.get_by_id(&user_id).await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        application::user::contracts::{
            MockJwtGeneratorContract, MockJwtValidatorContract, MockUserRepositoryContract,
        },
        domain::common::identifier::Identifier,
    };

    use super::*;
    use mockall::predicate;

    #[tokio::test]
    async fn test_valid_token_returns_user() {
        let mut user_repo = MockUserRepositoryContract::new();
        let jwt_generator = MockJwtGeneratorContract::new();
        let mut jwt_validator = MockJwtValidatorContract::new();

        let user_id = Identifier::new();
        let request = UserAuthenticateRequest {
            token: "test-token".to_string(),
        };

        jwt_validator
            .expect_validate_token()
            .times(1)
            .with(predicate::eq("test-token"))
            .returning(move |_| Ok(user_id));

        user_repo
            .expect_check_if_exists()
            .times(1)
            .with(predicate::eq(user_id))
            .returning(move |_| Box::pin(async move { Ok(true) }));

        user_repo
            .expect_get_by_id()
            .times(1)
            .with(predicate::eq(user_id))
            .returning(move |_| {
                let user = User::try_new(
                    user_id.clone(),
                    "test-user".to_string(),
                    "password".to_string(),
                )
                .unwrap();
                Box::pin(async move { Ok(user) })
            });

        let handler = UserRequestHandler::new(
            Arc::new(user_repo),
            Arc::new(jwt_generator),
            Arc::new(jwt_validator),
        );
        let result = handler.authenticate(request).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.id(), &user_id);
    }

    #[tokio::test]
    async fn test_invalid_token_returns_error() {
        let mut user_repo = MockUserRepositoryContract::new();
        let jwt_generator = MockJwtGeneratorContract::new();
        let mut jwt_validator = MockJwtValidatorContract::new();
        let request = UserAuthenticateRequest {
            token: "invalid-token".to_string(),
        };

        jwt_validator
            .expect_validate_token()
            .times(1)
            .with(predicate::eq("invalid-token"))
            .returning(|_| Err(UserError::new(UserErrorKind::InvalidCredentials)));

        user_repo.expect_get_by_id().never();

        let handler = UserRequestHandler::new(
            Arc::new(user_repo),
            Arc::new(jwt_generator),
            Arc::new(jwt_validator),
        );
        let result = handler.authenticate(request).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, UserError::new(UserErrorKind::InvalidCredentials));
    }

    #[tokio::test]
    async fn test_valid_token_no_matching_user_returns_error() {
        let mut user_repo = MockUserRepositoryContract::new();
        let jwt_generator = MockJwtGeneratorContract::new();
        let mut jwt_validator = MockJwtValidatorContract::new();

        let user_id = Identifier::new();
        let request = UserAuthenticateRequest {
            token: "test-token".to_string(),
        };

        let user_id_clone = user_id.clone();
        jwt_validator
            .expect_validate_token()
            .times(1)
            .with(predicate::eq("test-token"))
            .returning(move |_| Ok(user_id_clone.clone()));

        user_repo
            .expect_check_if_exists()
            .times(1)
            .with(predicate::eq(user_id))
            .returning(move |_| Box::pin(async move { Ok(false) }));

        let handler = UserRequestHandler::new(
            Arc::new(user_repo),
            Arc::new(jwt_generator),
            Arc::new(jwt_validator),
        );
        let result = handler.authenticate(request).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, UserError::new(UserErrorKind::UserNotFound));
    }
}
