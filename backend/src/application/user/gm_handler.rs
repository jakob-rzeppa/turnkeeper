// use std::sync::LazyLock;
// use crate::application::user::contracts::{JwtGeneratorTrait, JwtValidatorTrait};
// use crate::domain::error::Error;
//
// const GM_PASSWORD: LazyLock<String> = LazyLock::new(|| {
//     if cfg!(test) {
//         return "test-password".to_string();
//     }
//
//     std::env::var("GM_PASSWORD")
//         .expect("GM_PASSWORD environment variable is not set")
// });
//
// pub struct GmAuthHandler<JwtGenerator, JwtValidator>
// where
//     JwtGenerator: JwtGeneratorTrait,
//     JwtValidator: JwtValidatorTrait,
// {
//     jwt_generator: JwtGenerator,
//     jwt_validator: JwtValidator,
// }
//
// impl<JwtGenerator, JwtValidator> GmAuthHandler<JwtGenerator, JwtValidator>
// where
//     JwtGenerator: JwtGeneratorTrait,
//     JwtValidator: JwtValidatorTrait,
// {
//     pub fn new(jwt_generator: JwtGenerator, jwt_validator: JwtValidator) -> Self {
//         Self { jwt_generator, jwt_validator }
//     }
//
//     pub fn login(&self, request: LoginGmRequestDto) -> Result<TokenResponseDto, Error> {
//         if (request.password != *GM_PASSWORD) {
//             return Err(Error::InvalidCredentials {
//                 msg: "Wrong password".to_string(),
//             })
//         }
//
//         let token = self.jwt_generator.generate_gm_token()?;
//
//         Ok(TokenResponseDto { token })
//     }
//
//     pub fn authenticate(&self, token: BearerToken) -> Result<(), Error> {
//         self.jwt_validator.validate_gm_token(token)?;
//         Ok(())
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     mod login {
//         use crate::application::user::dto::LoginGmRequestDto;
//         use crate::application::user::gm_handler::GmAuthHandler;
//         use crate::domain::error::Error;
//
//         #[test]
//         fn test_valid_password_returns_token() {
//             let mut mock_jwt_generator = crate::application::user::contracts::MockJwtGeneratorTrait::new();
//             let mock_jwt_validator = crate::application::user::contracts::MockJwtValidatorTrait::new();
//
//             mock_jwt_generator
//                 .expect_generate_gm_token()
//                 .times(1)
//                 .returning(|| Ok("test-token".to_string()));
//
//             let handler = GmAuthHandler::new(mock_jwt_generator, mock_jwt_validator);
//             let request = LoginGmRequestDto { password: "test-password".to_string() };
//             let result = handler.login(request);
//
//             assert!(result.is_ok());
//             let token_response = result.unwrap();
//             assert_eq!(token_response.token, "test-token");
//         }
//
//         #[test]
//         fn test_invalid_password_does_not_call_jwt_generator_and_returns_error() {
//             let mut mock_jwt_generator = crate::application::user::contracts::MockJwtGeneratorTrait::new();
//             let mock_jwt_validator = crate::application::user::contracts::MockJwtValidatorTrait::new();
//
//             mock_jwt_generator.expect_generate_gm_token().never();
//
//             let handler = GmAuthHandler::new(mock_jwt_generator, mock_jwt_validator);
//             let request = LoginGmRequestDto { password: "invalid".to_string() };
//             let result = handler.login(request);
//
//             assert!(result.is_err());
//             let err = result.unwrap_err();
//             assert_eq!(err, Error::InvalidCredentials { msg: "Wrong password".to_string() });
//         }
//     }
//
//     mod authenticate {
//         use mockall::predicate;
//         use crate::application::user::dto::{BearerToken};
//         use crate::application::user::gm_handler::GmAuthHandler;
//         use crate::domain::error::Error;
//
//         #[test]
//         fn test_valid_token_returns_correct_response() {
//             let mock_jwt_generator = crate::application::user::contracts::MockJwtGeneratorTrait::new();
//             let mut mock_jwt_validator = crate::application::user::contracts::MockJwtValidatorTrait::new();
//
//             let token = "test-token".to_string();
//
//             mock_jwt_validator
//                 .expect_validate_gm_token()
//                 .times(1)
//                 .with(predicate::eq(BearerToken { token: token.clone() }))
//                 .returning(|_| Ok(()));
//
//             let handler = GmAuthHandler::new(mock_jwt_generator, mock_jwt_validator);
//             let request = BearerToken { token: token.clone() };
//             let res = handler.authenticate(request);
//
//             assert!(res.is_ok());
//         }
//
//         #[test]
//         fn test_invalid_token_returns_correct_error() {
//             let mock_jwt_generator = crate::application::user::contracts::MockJwtGeneratorTrait::new();
//             let mut mock_jwt_validator = crate::application::user::contracts::MockJwtValidatorTrait::new();
//
//             mock_jwt_validator
//                 .expect_validate_gm_token()
//                 .times(1)
//                 .with(predicate::eq(BearerToken { token: "invalid-test-token".to_string() }))
//                 .returning(move |_| Err(Error::InvalidCredentials { msg: "Wrong password".to_string() }));
//
//             let handler = GmAuthHandler::new(mock_jwt_generator, mock_jwt_validator);
//             let request = BearerToken { token: "invalid-test-token".to_string() };
//             let res = handler.authenticate(request);
//
//             assert!(res.is_err());
//             let err = res.unwrap_err();
//             assert_eq!(err, Error::InvalidCredentials { msg: "Wrong password".to_string() });
//         }
//     }
// }