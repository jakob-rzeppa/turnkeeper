pub struct RegisterUserRequestDto {
    pub name: String,
    pub password: String,
}

pub struct LoginUserRequestDto {
    pub name: String,
    pub password: String,
}

pub struct LoginGmRequestDto {
    pub password: String,
}

pub struct TokenResponseDto {
    pub token: String,
}