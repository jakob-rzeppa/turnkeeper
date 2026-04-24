//! # User Responses
//!
//! Response DTOs for user authentication.

use crate::domain::common::identifier::Identifier;

#[derive(Debug)]
pub struct UserTokenResponse {
    pub token: String,
}

#[derive(Debug)]
pub struct UserAuthenticationResponse {
    pub user_id: Identifier,
}

#[derive(Debug)]
pub struct UserListResponse {
    pub users: Vec<String>,
}
