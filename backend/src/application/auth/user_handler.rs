use uuid::Uuid;
use crate::application::auth::dto::{BearerToken, LoginUserRequestDto, RegisterUserRequestDto, TokenResponseDto};
use crate::domain::auth::entities::User;
use crate::domain::auth::jwt::{JwtGeneratorTrait, JwtValidatorTrait};
use crate::domain::error::Error;
use crate::domain::auth::traits::UserRepositoryTrait;

pub struct UserAuthHandler<UserRepo, JwtGenerator, JwtValidator>
where
    UserRepo: UserRepositoryTrait,
    JwtGenerator: JwtGeneratorTrait,
    JwtValidator: JwtValidatorTrait,
{
    user_repo: UserRepo,
    jwt_generator: JwtGenerator,
    jwt_validator: JwtValidator,
}

impl<UserRepo, JwtGenerator, JwtValidator> UserAuthHandler<UserRepo, JwtGenerator, JwtValidator>
where
    UserRepo: UserRepositoryTrait,
    JwtGenerator: JwtGeneratorTrait,
    JwtValidator: JwtValidatorTrait,
{
    pub fn new(user_repo: UserRepo, jwt_generator: JwtGenerator, jwt_validator: JwtValidator) -> Self {
        Self { user_repo, jwt_generator, jwt_validator }
    }

    pub async fn register(&self, request: RegisterUserRequestDto) -> Result<TokenResponseDto, Error> {
        let user = User::try_new(
            Uuid::new_v4(),
            request.name,
            request.password,
        )?;

        self.user_repo.save(user.clone()).await?;

        let token = self.jwt_generator.generate_user_token(user.id().clone())?;
        Ok(TokenResponseDto {
            token,
        })
    }

    pub async fn login(&self, request: LoginUserRequestDto) -> Result<TokenResponseDto, Error> {
        let user: User = self.user_repo.get_by_name(request.name.clone()).await?;

        user.check_password(request.password)?;

        let token = self.jwt_generator.generate_user_token(user.id().clone())?;
        Ok(TokenResponseDto {
            token,
        })
    }

    pub async fn authenticate(&self, token: BearerToken) -> Result<User, Error> {
        let user_id = self.jwt_validator.validate_user_token(token)?;

        let user = self.user_repo.get_by_id(user_id).await?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    mod login {
        use uuid::Uuid;
        use crate::application::auth::user_handler::UserAuthHandler;
        use crate::domain::auth::jwt::{MockJwtGeneratorTrait, MockJwtValidatorTrait};
        use crate::domain::auth::traits::MockUserRepositoryTrait;
        use crate::application::auth::dto::LoginUserRequestDto;
        use crate::domain::auth::entities::User;
        use crate::domain::error::Error;

        #[tokio::test]
        async fn test_valid_login_returns_token() {
            let mut user_repo = MockUserRepositoryTrait::new();
            let mut jwt_generator = MockJwtGeneratorTrait::new();
            let jwt_validator = MockJwtValidatorTrait::new();

            let name = "testuser".to_string();
            let password = "password".to_string();
            let request = LoginUserRequestDto { name: name.clone(), password: password.clone() };

            let user_id = Uuid::new_v4();
            let user = User::try_new(user_id.clone(), name.clone(), password.clone()).unwrap();
            user_repo
                .expect_get_by_name()
                .times(1)
                .returning(move |_| Ok(user.clone()));

            jwt_generator.expect_generate_user_token()
                .times(1)
                .returning(|_| Ok("login-token".to_string()));

            let handler = UserAuthHandler::new(user_repo, jwt_generator, jwt_validator);
            let result = handler.login(request).await;

            assert!(result.is_ok());
            let token_response = result.unwrap();
            assert_eq!(token_response.token, "login-token");
        }

        #[tokio::test]
        async fn test_invalid_password_login_returns_error() {
            let mut user_repo = MockUserRepositoryTrait::new();
            let mut jwt_generator = MockJwtGeneratorTrait::new();
            let jwt_validator = MockJwtValidatorTrait::new();

            let name = "testuser".to_string();
            let password = "invalid-password".to_string();
            let request = LoginUserRequestDto { name: name.clone(), password: password.clone() };

            let user_id = Uuid::new_v4();
            let user = User::try_new(user_id.clone(), name.clone(), "real-password".to_string()).unwrap();
            user_repo
                .expect_get_by_name()
                .times(1)
                .returning(move |_| Ok(user.clone()));

            jwt_generator.expect_generate_user_token()
                .never();

            let handler = UserAuthHandler::new(user_repo, jwt_generator, jwt_validator);
            let result = handler.login(request).await;

            assert!(result.is_err());
            let err = result.unwrap_err();
            assert_eq!(err, Error::InvalidCredentials { msg: "Wrong password".to_string() });
        }
    }

    mod register {
        use crate::application::auth::user_handler::UserAuthHandler;
        use crate::domain::auth::jwt::{MockJwtGeneratorTrait, MockJwtValidatorTrait};
        use crate::domain::auth::traits::MockUserRepositoryTrait;

        #[tokio::test]
        async fn test_valid_call_save_and_return_token() {
            use crate::application::auth::dto::RegisterUserRequestDto;

            let mut user_repo = MockUserRepositoryTrait::new();
            let mut jwt_generator = MockJwtGeneratorTrait::new();
            let jwt_validator = MockJwtValidatorTrait::new();

            // Prepare test data
            let name = "testuser".to_string();
            let password = "password".to_string();
            let request = RegisterUserRequestDto { name: name.clone(), password: password.clone() };

            // We don't care about the actual user, so use any()
            user_repo.expect_save()
                .times(1)
                .returning(|_| Ok(()) );

            // The token we expect to be returned
            jwt_generator.expect_generate_user_token()
                .times(1)
                .returning(|_| Ok("test-token".to_string()));

            let handler = UserAuthHandler::new(user_repo, jwt_generator, jwt_validator);
            let result = handler.register(request).await;

            assert!(result.is_ok());
            let token_response = result.unwrap();
            assert_eq!(token_response.token, "test-token");
        }
    }

    mod authenticate {
        use uuid::Uuid;
        use crate::application::auth::user_handler::UserAuthHandler;
        use crate::domain::auth::jwt::{MockJwtGeneratorTrait, MockJwtValidatorTrait};
        use crate::domain::auth::traits::MockUserRepositoryTrait;
        use crate::application::auth::dto::BearerToken;
        use crate::domain::auth::entities::User;
        use crate::domain::error::Error;

        #[tokio::test]
        async fn test_valid_token_returns_user() {
            let mut user_repo = MockUserRepositoryTrait::new();
            let jwt_generator = MockJwtGeneratorTrait::new();
            let mut jwt_validator = MockJwtValidatorTrait::new();

            let user_id = Uuid::new_v4();
            let user = User::try_new(user_id.clone(), "testuser".to_string(), "password".to_string()).unwrap();
            let token = BearerToken::new("valid-token".to_string());

            let user_id_clone = user_id.clone();
            jwt_validator.expect_validate_user_token()
                .times(1)
                .returning(move |_| Ok(user_id_clone.clone()));

            user_repo.expect_get_by_id()
                .times(1)
                .returning(move |_| Ok(user.clone()));

            let handler = UserAuthHandler::new(user_repo, jwt_generator, jwt_validator);
            let result = handler.authenticate(token).await;

            assert!(result.is_ok());
            let returned_user = result.unwrap();
            assert_eq!(returned_user.id().clone(), user_id);
        }

        #[tokio::test]
        async fn test_invalid_token_returns_error() {
            let mut user_repo = MockUserRepositoryTrait::new();
            let jwt_generator = MockJwtGeneratorTrait::new();
            let mut jwt_validator = MockJwtValidatorTrait::new();

            let token = BearerToken::new("invalid-token".to_string());

            jwt_validator.expect_validate_user_token()
                .times(1)
                .returning(|_| Err(Error::InvalidCredentials { msg: "Invalid token".to_string() }));

            user_repo.expect_get_by_id()
                .never();

            let handler = UserAuthHandler::new(user_repo, jwt_generator, jwt_validator);
            let result = handler.authenticate(token).await;

            assert!(result.is_err());
            let err = result.unwrap_err();
            assert_eq!(err, Error::InvalidCredentials { msg: "Invalid token".to_string() });
        }
    }
}