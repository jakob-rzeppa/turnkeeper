pub struct RegisterRequestDto {
    pub name: String,
    pub password: String,
}

pub struct LoginRequestDto {
    pub name: String,
    pub password: String,
}

pub struct TokenResponseDto {
    pub token: String,
}