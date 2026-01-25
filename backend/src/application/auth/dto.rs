use crate::domain::error::Error;

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

#[derive(Debug)]
pub struct BearerToken {
    pub token: String,
}

impl BearerToken {
    pub fn new(token: String) -> BearerToken {
        BearerToken { token }
    }

    /// Extracts the token from a String in the form "Bearer <token>"
    pub fn from_header_string(header: String) -> Result<Self, Error> {
        if !header.contains("Bearer ") {
            return Err(Error::InvalidCredentials {
                msg: "Invalid bearer header".to_string(),
            });
        }

        let token = &header[7..header.len()];

        Ok(Self { token: String::from(token) })
    }
}

#[cfg(test)]
mod tests {
    use crate::application::auth::dto::BearerToken;
    use crate::domain::error::Error;

    #[test]
    fn test_bearer_token_from_string() {
        let header = "Bearer 89afioh839hprauhfia";
        let token = BearerToken::from_header_string(header.to_string());

        assert!(token.is_ok());
        let token = token.unwrap();
        assert_eq!("89afioh839hprauhfia", token.token);
    }

    #[test]
    fn test_bearer_token_from_string_no_bearer() {
        let header = "89afioh839hprauhfia";

        let token = BearerToken::from_header_string(header.to_string());

        assert!(token.is_err());
        let err = token.unwrap_err();
        assert_eq!(err, Error::InvalidCredentials { msg: "Invalid bearer header".to_string()});
    }

    #[test]
    fn test_bearer_token_from_string_no_whitespace() {
        let header = "Bearer89afioh839hprauhfia";

        let token = BearerToken::from_header_string(header.to_string());

        assert!(token.is_err());
        let err = token.unwrap_err();
        assert_eq!(err, Error::InvalidCredentials { msg: "Invalid bearer header".to_string()});
    }
}