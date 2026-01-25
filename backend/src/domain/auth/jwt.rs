use uuid::Uuid;
use crate::application::auth::dto::BearerToken;
use crate::domain::error::Error;

#[mockall::automock]
pub trait JwtGeneratorTrait {
    fn generate_user_token(&self, user_id: Uuid) -> Result<String, Error>;
    fn generate_gm_token(&self) -> Result<String, Error>;
}

#[mockall::automock]
pub trait JwtValidatorTrait {
    fn validate_user_token(&self, bearer_token: BearerToken) -> Result<Uuid, Error>;
    fn validate_gm_token(&self, bearer_token: BearerToken) -> Result<(), Error>;
}