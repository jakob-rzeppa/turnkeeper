use crate::application::gm::contracts::{GmJwtValidatorContract};
use crate::application::gm::requests::{GmAuthenticateRequest};
use crate::domain::gm::error::GmError;

pub struct GmAuthenticateRequestHandler<JwtValidator>
where
    JwtValidator: GmJwtValidatorContract + 'static,
{
    jwt: JwtValidator,
}

impl<JwtValidator> GmAuthenticateRequestHandler<JwtValidator>
where
    JwtValidator: GmJwtValidatorContract + 'static,
{
    pub fn new(jwt: JwtValidator) -> Self {
        Self { jwt }
    }

    pub async fn authenticate(&self, request: GmAuthenticateRequest) -> Result<(), GmError> {
        self.jwt.validate_token(&request.token)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;
    use crate::application::gm::contracts::MockGmJwtValidatorContract;
    use crate::application::gm::request_handlers::authenticate::GmAuthenticateRequestHandler;
    use crate::application::gm::requests::GmAuthenticateRequest;
    use crate::domain::gm::error::{GmError, GmErrorKind};

    #[tokio::test]
    async fn test_valid_token_returns_correct_response() {
        let mut mock_jwt_validator = MockGmJwtValidatorContract::new();

        let token = "test-token".to_string();

        mock_jwt_validator
            .expect_validate_token()
            .times(1)
            .with(predicate::eq(token.clone()))
            .returning(|_| Ok(()));

        let handler = GmAuthenticateRequestHandler::new(mock_jwt_validator);
        let request = GmAuthenticateRequest { token: token.clone() };
        let res = handler.authenticate(request).await;

        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_token_returns_correct_error() {
        let mut mock_jwt_validator = MockGmJwtValidatorContract::new();

        mock_jwt_validator
            .expect_validate_token()
            .times(1)
            .with(predicate::eq("invalid-test-token".to_string()))
            .returning(move |_| Err(GmError::new(GmErrorKind::InvalidCredentials)));

        let handler = GmAuthenticateRequestHandler::new(mock_jwt_validator);
        let request = GmAuthenticateRequest { token: "invalid-test-token".to_string() };
        let res = handler.authenticate(request).await;

        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err, GmError::new(GmErrorKind::InvalidCredentials));
    }
}