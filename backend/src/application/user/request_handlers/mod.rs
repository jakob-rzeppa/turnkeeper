use std::sync::Arc;

use crate::application::user::contracts::{
    JwtGeneratorContract, JwtValidatorContract, UserRepositoryContract,
};

pub mod authenticate;
pub mod login;
pub mod register;
pub mod user_list;

pub struct UserRequestHandler<
    UserRepository: UserRepositoryContract,
    JwtGenerator: JwtGeneratorContract,
    JwtValidator: JwtValidatorContract,
> {
    user_repository: Arc<UserRepository>,
    jwt_generator: Arc<JwtGenerator>,
    jwt_validator: Arc<JwtValidator>,
}

impl<
    UserRepository: UserRepositoryContract,
    JwtGenerator: JwtGeneratorContract,
    JwtValidator: JwtValidatorContract,
> UserRequestHandler<UserRepository, JwtGenerator, JwtValidator>
{
    pub fn new(
        user_repository: Arc<UserRepository>,
        jwt_generator: Arc<JwtGenerator>,
        jwt_validator: Arc<JwtValidator>,
    ) -> Self {
        Self {
            user_repository,
            jwt_generator,
            jwt_validator,
        }
    }
}

impl<
    UserRepository: UserRepositoryContract,
    JwtGenerator: JwtGeneratorContract,
    JwtValidator: JwtValidatorContract,
> Clone for UserRequestHandler<UserRepository, JwtGenerator, JwtValidator>
{
    fn clone(&self) -> Self {
        Self {
            user_repository: self.user_repository.clone(),
            jwt_generator: self.jwt_generator.clone(),
            jwt_validator: self.jwt_validator.clone(),
        }
    }
}
