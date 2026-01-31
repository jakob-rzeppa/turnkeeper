use crate::domain::gm::error::GmError;

#[mockall::automock]
pub trait GmJwtGeneratorContract {
    fn generate_token(&self) -> Result<String, GmError>;
}

#[mockall::automock]
pub trait GmJwtValidatorContract {
    fn validate_token(&self, token: &str) -> Result<(), GmError>;
}