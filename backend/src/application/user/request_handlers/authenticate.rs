use crate::application::user::contracts::{UserJwtValidatorContract, UserRepositoryContract};
use crate::application::user::requests::{UserAuthenticateRequest};
use crate::application::user::responses::{UserAuthenticationResponse};
use crate::domain::error::Error;

pub struct UserAuthenticateRequestHandler<UserRepository, JwtValidator>
where
    UserRepository: UserRepositoryContract + 'static,
    JwtValidator: UserJwtValidatorContract + 'static,
{
    repository: UserRepository,
    jwt: JwtValidator,
}

impl<UserRepository, JwtValidator> UserAuthenticateRequestHandler<UserRepository, JwtValidator>
where
    UserRepository: UserRepositoryContract + 'static,
    JwtValidator: UserJwtValidatorContract + 'static,
{
    pub fn new(repository: UserRepository, jwt: JwtValidator) -> Self {
        Self { repository, jwt }
    }

    pub async fn authenticate(&self, request: UserAuthenticateRequest) -> Result<UserAuthenticationResponse, Error> {
        let user_id = self.jwt.validate_token(&request.token)?;

        if !self.repository.check_if_exists(&user_id).await? {
            return Err(Error::NotFound { msg: "User does not exist".to_string() });
        }

        Ok(UserAuthenticationResponse {
            user_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;
    use uuid::Uuid;
    use crate::application::user::contracts::{MockUserJwtValidatorContract, MockUserRepositoryContract};
    use crate::application::user::request_handlers::authenticate::UserAuthenticateRequestHandler;
    use crate::application::user::requests::UserAuthenticateRequest;
    use crate::domain::error::Error;

    #[tokio::test]
    async fn test_valid_token_returns_user() {
        let mut user_repo = MockUserRepositoryContract::new();
        let mut jwt_validator = MockUserJwtValidatorContract::new();

        let user_id = Uuid::new_v4();
        let request = UserAuthenticateRequest {
            token: "test-token".to_string(),
        };

        jwt_validator.expect_validate_token()
            .times(1)
            .with(predicate::eq("test-token"))
            .returning(move |_| Ok(user_id));

        user_repo.expect_check_if_exists()
            .times(1)
            .with(predicate::eq(user_id))
            .returning(move |_| Ok(true));

        let handler = UserAuthenticateRequestHandler::new(user_repo, jwt_validator);
        let result = handler.authenticate(request).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.user_id, user_id);
    }

    #[tokio::test]
    async fn test_invalid_token_returns_error() {
        let mut user_repo = MockUserRepositoryContract::new();
        let mut jwt_validator = MockUserJwtValidatorContract::new();
        let request = UserAuthenticateRequest {
            token: "invalid-token".to_string(),
        };

        jwt_validator.expect_validate_token()
            .times(1)
            .with(predicate::eq("invalid-token"))
            .returning(|_| Err(Error::InvalidCredentials { msg: "Invalid token".to_string() }));

        user_repo.expect_get_by_id()
            .never();

        let handler = UserAuthenticateRequestHandler::new(user_repo, jwt_validator);
        let result = handler.authenticate(request).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, Error::InvalidCredentials { msg: "Invalid token".to_string() });
    }

    #[tokio::test]
    async fn test_valid_token_no_matching_user_returns_error() {
        let mut user_repo = MockUserRepositoryContract::new();
        let mut jwt_validator = MockUserJwtValidatorContract::new();

        let user_id = Uuid::new_v4();
        let request = UserAuthenticateRequest {
            token: "test-token".to_string(),
        };

        let user_id_clone = user_id.clone();
        jwt_validator.expect_validate_token()
            .times(1)
            .with(predicate::eq("test-token"))
            .returning(move |_| Ok(user_id_clone.clone()));

        user_repo.expect_check_if_exists()
            .times(1)
            .with(predicate::eq(user_id))
            .returning(move |_| Ok(false));

        let handler = UserAuthenticateRequestHandler::new(user_repo, jwt_validator);
        let result = handler.authenticate(request).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, Error::NotFound { msg: "User does not exist".to_string() });
    }
}