use std::sync::LazyLock;
use crate::application::gm::contracts::GmJwtGeneratorContract;
use crate::application::gm::requests::GmLoginRequest;
use crate::application::gm::responses::GmTokenResponse;
use crate::domain::error::Error;

const GM_PASSWORD: LazyLock<String> = LazyLock::new(|| {
    if cfg!(test) {
        return "test-password".to_string();
    }

    std::env::var("GM_PASSWORD")
        .expect("GM_PASSWORD environment variable is not set")
});

pub struct GmLoginRequestHandler<JwtGenerator>
where
    JwtGenerator: GmJwtGeneratorContract + 'static,
{
    jwt: JwtGenerator,
}

impl<JwtGenerator> GmLoginRequestHandler<JwtGenerator>
where
    JwtGenerator: GmJwtGeneratorContract + 'static,
{
    pub fn new(jwt: JwtGenerator) -> Self {
        Self { jwt }
    }

    pub async fn login(&self, request: GmLoginRequest) -> Result<GmTokenResponse, Error> {
        if *GM_PASSWORD != request.password {
            return Err(Error::InvalidCredentials { msg: "Wrong password".to_string() });
        }

        let token = self.jwt.generate_token()?;

        Ok(GmTokenResponse {
            token,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::application::gm::contracts::MockGmJwtGeneratorContract;
    use crate::application::gm::request_handlers::login::GmLoginRequestHandler;
    use crate::application::gm::requests::GmLoginRequest;
    use crate::domain::error::Error;

    #[tokio::test]
    async fn test_valid_password_returns_token() {
        let mut mock_jwt_generator = MockGmJwtGeneratorContract::new();

        mock_jwt_generator
            .expect_generate_token()
            .times(1)
            .returning(|| Ok("test-token".to_string()));

        let handler = GmLoginRequestHandler::new(mock_jwt_generator);
        let request = GmLoginRequest { password: "test-password".to_string() };
        let result = handler.login(request).await;

        assert!(result.is_ok());
        let token_response = result.unwrap();
        assert_eq!(token_response.token, "test-token");
    }

    #[tokio::test]
    async fn test_invalid_password_does_not_call_jwt_generator_and_returns_error() {
        let mut mock_jwt_generator = MockGmJwtGeneratorContract::new();

        mock_jwt_generator.expect_generate_token().never();

        let handler = GmLoginRequestHandler::new(mock_jwt_generator);
        let request = GmLoginRequest { password: "invalid".to_string() };
        let result = handler.login(request).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err, Error::InvalidCredentials { msg: "Wrong password".to_string() });
    }
}