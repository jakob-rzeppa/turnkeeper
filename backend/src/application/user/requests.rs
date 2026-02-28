//! # User Requests
//!
//! Request DTOs for user authentication and registration.

pub struct UserRegisterRequest {
    pub name: String,
    pub password: String,
}

pub struct UserLoginRequest {
    pub name: String,
    pub password: String,
}

pub struct UserAuthenticateRequest {
    pub token: String,
}