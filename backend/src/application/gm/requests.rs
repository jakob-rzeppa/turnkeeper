pub struct GmRegisterRequest {
    pub password: String,
}

pub struct GmLoginRequest {
    pub password: String,
}

pub struct GmAuthenticateRequest {
    pub token: String,
}