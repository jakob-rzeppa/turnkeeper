
pub struct RegisterRequest {
    pub name: String,
    pub password: String,
}

pub struct LoginRequest {
    pub name: String,
    pub password: String,
}

pub struct AuthenticateRequest {
    pub token: String,
}