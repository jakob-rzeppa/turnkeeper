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
pub trait JwtGeneratorTrait {
    fn generate_user_token(&self, user_id: &Uuid) -> Result<String, Error>;
    fn generate_gm_token(&self) -> Result<String, Error>;
}

#[mockall::automock]
pub trait JwtValidatorTrait {
    fn validate_user_token(&self, bearer_token: &str) -> Result<Uuid, Error>;
    fn validate_gm_token(&self, bearer_token: &str) -> Result<(), Error>;
}