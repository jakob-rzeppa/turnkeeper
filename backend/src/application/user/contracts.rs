use uuid::Uuid;
use crate::domain::user::entities::User;
use crate::domain::user::error::UserError;

#[mockall::automock]
pub trait UserRepositoryContract {
    async fn check_if_exists(&self, id: &Uuid) -> Result<bool, UserError>;

    async fn get_by_id(&self, id: &Uuid) -> Result<User, UserError>;

    async fn get_by_name(&self, name: &str) -> Result<User, UserError>;

    /// Save needs to check that the name is unique
    async fn save(&self, user: &User) -> Result<(), UserError>;
}

#[mockall::automock]
pub trait UserJwtGeneratorContract {
    fn generate_token(&self, user_id: &Uuid) -> Result<String, UserError>;
}

#[mockall::automock]
pub trait UserJwtValidatorContract {
    fn validate_token(&self, bearer_token: &str) -> Result<Uuid, UserError>;
}