//! # GM Responses
//!
//! Response DTOs for GM authentication.

/// Response containing a JWT token after GM login.
#[derive(Debug)]
pub struct GmTokenResponse {
    pub token: String,
}