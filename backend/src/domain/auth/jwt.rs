use crate::domain::error::Error;
use crate::domain::value_object::identity::Identity;

pub trait JwtGeneratorTrait {
    fn generate_user_jwt(&self, user_id: Identity) -> Result<String, Error>;
    fn generate_gm_jwt(&self) -> Result<String, Error>;
}

pub trait JwtValidatorTrait {
    fn validate_user_jwt(&self) -> Result<Identity, Error>;
    fn validate_gm_jwt(&self) -> Result<(), Error>;
}