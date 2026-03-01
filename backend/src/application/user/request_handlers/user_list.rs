use std::sync::Arc;
use crate::application::user::contracts::UserRepositoryContract;
use crate::domain::user::error::UserError;
use crate::domain::user::projections::UserListProjection;

pub struct UserListRequestHandler<UserRepository>
where
    UserRepository: UserRepositoryContract,
{
    repository: Arc<UserRepository>,
}

impl<UserRepository> UserListRequestHandler<UserRepository>
where
    UserRepository: UserRepositoryContract,
{
    pub fn new(repository: Arc<UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn list(&self) -> Result<Vec<UserListProjection>, UserError> {
        let users = self.repository.get_all().await?;
        let user_list_projections: Vec<UserListProjection> = users.iter().map(|user| UserListProjection::from(user)).collect();
        Ok(user_list_projections)
    }
}