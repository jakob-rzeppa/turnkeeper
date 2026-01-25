use crate::application::auth::dto::{BearerToken, LoginGmRequestDto, LoginUserRequestDto, RegisterUserRequestDto, TokenResponseDto};
use crate::domain::auth::jwt::{JwtGeneratorTrait, JwtValidatorTrait};
use crate::domain::entity::user::User;
use crate::domain::error::Error;
use crate::domain::repository::UserRepositoryTrait;

pub struct UserAuthHandler<UserRepo, JwtGenerator, JwtValidator>
where
    UserRepo: UserRepositoryTrait,
    JwtGenerator: JwtValidatorTrait,
    JwtValidator: JwtGeneratorTrait,
{
    user_repo: UserRepo,
    jwt_generator: JwtGenerator,
    jwt_validator: JwtValidator,
}

impl<UserRepo, JwtGenerator, JwtValidator> UserAuthHandler<UserRepo, JwtGenerator, JwtValidator>
where
    UserRepo: UserRepositoryTrait,
    JwtGenerator: JwtValidatorTrait,
    JwtValidator: JwtGeneratorTrait,
{
    pub fn new(user_repo: UserRepo, jwt_generator: JwtGenerator, jwt_validator: JwtValidator) -> Self {
        Self { user_repo, jwt_generator, jwt_validator }
    }

    pub fn register(request: RegisterUserRequestDto) -> Result<TokenResponseDto, Error> {
        Err(Error::NotImplemented)
    }

    pub fn login(request: LoginUserRequestDto) -> Result<TokenResponseDto, Error> {
        Err(Error::NotImplemented)
    }

    pub fn authenticate(token: BearerToken) -> Result<User, Error> {
        Err(Error::NotImplemented)
    }
}