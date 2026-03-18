//! # GM Requests
//!
//! Request DTOs for GM authentication.

pub struct GmLoginRequest {
    pub password: String,
}

pub struct GmAuthenticateRequest {
    pub token: String,
}