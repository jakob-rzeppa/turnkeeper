use crate::application::auth::dto::{BearerToken, LoginGmRequestDto, TokenResponseDto};
use crate::domain::auth::jwt::{JwtGeneratorTrait, JwtValidatorTrait};
use crate::domain::error::Error;

pub struct UserAuthHandler<JwtGenerator, JwtValidator>
where
    JwtGenerator: JwtGeneratorTrait,
    JwtValidator: JwtValidatorTrait,
{
    jwt_generator: JwtGenerator,
    jwt_validator: JwtValidator,
}

impl<JwtGenerator, JwtValidator> UserAuthHandler<JwtGenerator, JwtValidator>
where
    JwtGenerator: JwtGeneratorTrait,
    JwtValidator: JwtValidatorTrait,
{
    pub fn new(jwt_generator: JwtGenerator, jwt_validator: JwtValidator) -> Self {
        Self { jwt_generator, jwt_validator }
    }

    pub fn login(request: LoginGmRequestDto) -> Result<TokenResponseDto, Error> {
        Err(Error::NotImplemented)
    }

    pub fn authenticate(token: BearerToken) -> Result<(), Error> {
        Err(Error::NotImplemented)
    }
}