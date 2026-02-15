use std::sync::Arc;
use crate::infrastructure::auth::gm_jwt::{GmJwtGenerator, GmJwtValidator};
use crate::infrastructure::auth::user_jwt::{UserJwtGenerator, UserJwtValidator};

pub mod user_jwt;
pub mod gm_jwt;

pub struct AuthManager {
    gm_jwt_generator: Arc<GmJwtGenerator>,
    gm_jwt_validator: Arc<GmJwtValidator>,

    user_jwt_generator: Arc<UserJwtGenerator>,
    user_jwt_validator: Arc<UserJwtValidator>,
}

impl AuthManager {
    pub fn new() -> Self {
        Self {
            gm_jwt_generator: Arc::new(GmJwtGenerator {}),
            gm_jwt_validator: Arc::new(GmJwtValidator {}),

            user_jwt_generator: Arc::new(UserJwtGenerator {}),
            user_jwt_validator: Arc::new(UserJwtValidator {}),
        }
    }
    
    pub fn gm_jwt_generator(&self) -> Arc<GmJwtGenerator> {
        self.gm_jwt_generator.clone()
    }

    pub fn gm_jwt_validator(&self) -> Arc<GmJwtValidator> {
        self.gm_jwt_validator.clone()
    }

    pub fn user_jwt_generator(&self) -> Arc<UserJwtGenerator> {
        self.user_jwt_generator.clone()
    }

    pub fn user_jwt_validator(&self) -> Arc<UserJwtValidator> {
        self.user_jwt_validator.clone()
    }
}

impl Clone for AuthManager {
    fn clone(&self) -> Self {
        Self {
            gm_jwt_generator: self.gm_jwt_generator.clone(),
            gm_jwt_validator: self.gm_jwt_validator.clone(),

            user_jwt_generator: self.user_jwt_generator.clone(),
            user_jwt_validator: self.user_jwt_validator.clone(),
        }
    }
}