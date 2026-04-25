use crate::application::user::contracts::{
    JwtGeneratorContract, JwtValidatorContract, UserRepositoryContract,
};
use crate::application::user::request_handlers::UserRequestHandler;
use crate::domain::user::error::UserError;
use crate::domain::user::projections::UserListProjection;

impl<
    UserRepository: UserRepositoryContract,
    JwtGenerator: JwtGeneratorContract,
    JwtValidator: JwtValidatorContract,
> UserRequestHandler<UserRepository, JwtGenerator, JwtValidator>
{
    pub async fn list(&self) -> Result<Vec<UserListProjection>, UserError> {
        let users = self.user_repository.get_all().await?;
        let user_list_projections: Vec<UserListProjection> = users
            .iter()
            .map(|user| UserListProjection::from(user))
            .collect();
        Ok(user_list_projections)
    }
}
