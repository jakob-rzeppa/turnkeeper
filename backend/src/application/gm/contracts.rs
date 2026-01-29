use crate::domain::error::Error;

#[mockall::automock]
pub trait GmJwtGeneratorContract {
    fn generate_token(&self) -> Result<String, Error>;
}

#[mockall::automock]
pub trait GmJwtValidatorContract {
    fn validate_token(&self, token: &str) -> Result<(), Error>;
}