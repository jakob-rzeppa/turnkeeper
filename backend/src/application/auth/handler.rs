use crate::application::auth::dto::{LoginGmRequestDto, LoginUserRequestDto, RegisterUserRequestDto, TokenResponseDto};
use crate::domain::auth::jwt::JwtGeneratorTrait;
use crate::domain::error::Error;
use crate::domain::repository::UserRepositoryTrait;

pub struct AuthHandler<UserRepo: UserRepositoryTrait, JwtGenerator: JwtGeneratorTrait> {
    user_repo: UserRepo,
    jwt_generator: JwtGenerator,
}

impl<UserRepo: UserRepositoryTrait, JwtGenerator: JwtGeneratorTrait> AuthHandler<UserRepo, JwtGenerator> {
    /// Create a new AuthHandler
    pub fn new(user_repo: UserRepo, jwt_generator: JwtGenerator) -> Self {
        Self { user_repo, jwt_generator }
    }

    pub fn register_user(request: RegisterUserRequestDto) -> Result<TokenResponseDto, Error> {
        Err(Error::NotImplemented)
    }

    pub fn login_user(request: LoginUserRequestDto) -> Result<TokenResponseDto, Error> {
        Err(Error::NotImplemented)
    }

    pub fn login_gm(request: LoginGmRequestDto) -> Result<TokenResponseDto, Error> {
        Err(Error::NotImplemented)
    }
}