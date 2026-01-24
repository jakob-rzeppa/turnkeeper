use crate::application::auth::dto::{LoginRequestDto, RegisterRequestDto, TokenResponseDto};
use crate::error::ApplicationError;

pub struct AuthHandler {}

impl AuthHandler {
    pub fn register(request: RegisterRequestDto) -> Result<TokenResponseDto, ApplicationError> {
        Err(ApplicationError::NotImplemented)
    }

    pub fn login(request: LoginRequestDto) -> Result<TokenResponseDto, ApplicationError> {
        Err(ApplicationError::NotImplemented)
    }
}