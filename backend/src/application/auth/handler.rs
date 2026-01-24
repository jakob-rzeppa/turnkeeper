use crate::application::auth::dto::{LoginRequestDto, RegisterRequestDto, TokenResponseDto};
use crate::domain::error::Error;

pub struct AuthHandler {}

impl AuthHandler {
    pub fn register(request: RegisterRequestDto) -> Result<TokenResponseDto, Error> {
        Err(Error::NotImplemented)
    }

    pub fn login(request: LoginRequestDto) -> Result<TokenResponseDto, Error> {
        Err(Error::NotImplemented)
    }
}