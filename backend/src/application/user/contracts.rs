use uuid::Uuid;
use crate::domain::user::entities::User;
use crate::domain::error::Error;

#[mockall::automock]
pub trait UserRepositoryTrait {
    async fn check_if_exists(&self, id: &Uuid) -> Result<bool, Error>;

    async fn get_by_id(&self, id: &Uuid) -> Result<User, Error>;

    async fn get_by_name(&self, name: &str) -> Result<User, Error>;

    /// Save needs to check that the name is unique
    async fn save(&self, user: &User) -> Result<(), Error>;
}

#[mockall::automock]
pub trait UserJwtGeneratorContract {
    fn generate_token(&self, user_id: &Uuid) -> Result<String, Error>;
}

#[mockall::automock]
pub trait UserJwtValidatorContract {
    fn validate_token(&self, bearer_token: &str) -> Result<Uuid, Error>;
}