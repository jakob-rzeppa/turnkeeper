//! # User Responses
//!
//! Response DTOs for user authentication.

use crate::domain::game::value_objects::id::Id;

#[derive(Debug)]
pub struct UserTokenResponse {
    pub token: String,
}

#[derive(Debug)]
pub struct UserAuthenticationResponse {
    pub user_id: Id,
}

#[derive(Debug)]
pub struct UserListResponse {
    pub users: Vec<String>,
}