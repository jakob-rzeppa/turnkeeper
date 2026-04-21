//! # Authentication Module
//!
//! Handles JWT-based authentication for Users.

use std::sync::Arc;

use crate::infrastructure::auth::jwt::{JwtGenerator, JwtValidator};

pub mod jwt;
pub mod middleware;

/// Manages JWT authentication for both GMs and Users.
///
/// Provides access to JWT generators and validators through a unified interface.
/// All fields use `Arc` for efficient cloning across request handlers.
pub struct AuthManager {
    jwt_generator: Arc<JwtGenerator>,
    jwt_validator: Arc<JwtValidator>,
}

impl AuthManager {
    /// Creates a new authentication manager.
    ///
    /// Initializes all JWT generators and validators.
    pub fn new() -> Self {
        Self {
            jwt_generator: Arc::new(JwtGenerator {}),
            jwt_validator: Arc::new(JwtValidator {}),
        }
    }

    pub fn jwt_generator(&self) -> Arc<JwtGenerator> {
        self.jwt_generator.clone()
    }

    pub fn jwt_validator(&self) -> Arc<JwtValidator> {
        self.jwt_validator.clone()
    }
}

impl Clone for AuthManager {
    fn clone(&self) -> Self {
        Self {
            jwt_generator: self.jwt_generator.clone(),
            jwt_validator: self.jwt_validator.clone(),
        }
    }
}
