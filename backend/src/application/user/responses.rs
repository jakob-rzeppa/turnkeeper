use uuid::Uuid;

#[derive(Debug)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Debug)]
pub struct AuthenticationResponse {
    pub user_id: Uuid,
}