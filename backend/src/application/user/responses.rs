//! # User Responses
//!
//! Response DTOs for user authentication.

use uuid::Uuid;

#[derive(Debug)]
pub struct UserTokenResponse {
    pub token: String,
}

#[derive(Debug)]
pub struct UserAuthenticationResponse {
    pub user_id: Uuid,
}

#[derive(Debug)]
pub struct UserListResponse {
    pub users: Vec<String>,
}