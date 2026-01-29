use uuid::Uuid;
use crate::application::user::contracts::{UserJwtGeneratorContract, UserRepositoryContract};
use crate::application::user::requests::{UserRegisterRequest};
use crate::application::user::responses::UserTokenResponse;
use crate::domain::error::Error;
use crate::domain::user::entities::User;

pub struct UserRegisterRequestHandler<UserRepository, JwtGenerator>
where
    UserRepository: UserRepositoryContract + 'static,
    JwtGenerator: UserJwtGeneratorContract + 'static,
{
    repository: UserRepository,
    jwt: JwtGenerator,
}

impl<UserRepository, JwtGenerator> UserRegisterRequestHandler<UserRepository, JwtGenerator>
where
    UserRepository: UserRepositoryContract + 'static,
    JwtGenerator: UserJwtGeneratorContract + 'static,
{
    pub fn new(repository: UserRepository, jwt: JwtGenerator) -> Self {
        Self { repository, jwt }
    }

    pub async fn register(&self, request: UserRegisterRequest) -> Result<UserTokenResponse, Error> {
        let user = User::try_new(
            Uuid::new_v4(),
            request.name,
            request.password,
        )?;

        self.repository.save(&user).await?;

        let token = self.jwt.generate_token(user.id())?;
        Ok(UserTokenResponse {
            token,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::application::user::contracts::{MockUserJwtGeneratorContract, MockUserRepositoryContract};
    use super::*;

    #[tokio::test]
    async fn test_valid_call_save_and_return_token() {
        let mut user_repo = MockUserRepositoryContract::new();
        let mut jwt_generator = MockUserJwtGeneratorContract::new();

        // Prepare test data
        let name = "test-user".to_string();
        let password = "password".to_string();
        let request = UserRegisterRequest { name: name.clone(), password: password.clone() };

        // We don't care about the actual user, so use any()
        user_repo.expect_save()
            .times(1)
            .returning(|_| Ok(()) );

        // The token we expect to be returned
        jwt_generator.expect_generate_token()
            .times(1)
            .returning(|_| Ok("test-token".to_string()));

        let handler = UserRegisterRequestHandler::new(user_repo, jwt_generator);
        let result = handler.register(request).await;

        assert!(result.is_ok());
        let token_response = result.unwrap();
        assert_eq!(token_response.token, "test-token");
    }
}