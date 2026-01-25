use crate::application::auth::dto::BearerToken;
use crate::domain::error::Error;
use crate::domain::value_object::identity::Identity;

#[mockall::automock]
pub trait JwtGeneratorTrait {
    fn generate_user_token(&self, user_id: Identity) -> Result<String, Error>;
    fn generate_gm_token(&self) -> Result<String, Error>;
}

#[mockall::automock]
pub trait JwtValidatorTrait {
    fn validate_user_token(&self, bearer_token: BearerToken) -> Result<Identity, Error>;
    fn validate_gm_token(&self, bearer_token: BearerToken) -> Result<(), Error>;
}