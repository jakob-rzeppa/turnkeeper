use axum::extract::{State};
use backend_derive::{JsonRequest, JsonResponse};
use serde::{Serialize, Deserialize};
use serde_valid::Validate;
use crate::error::{HttpError};
use crate::{AppState};
use crate::infrastructure::auth::jwt::generate_user_jwt;
use crate::infrastructure::repository::user::{create_user, UserCreateInformation};
use crate::infrastructure::repository::user::get_id_by_name_if_password;

#[derive(Deserialize, Validate, JsonRequest, Debug)]
pub struct LoginRequest {
    #[validate(min_length = 1)]
    name: String,
    #[validate(min_length = 1)]
    password: String
}

#[derive(Serialize, JsonResponse, Debug)]
pub struct LoginResponse {
    token: String,
}

/// POST /user/login
///
/// authenticates a user via username and password and returns a JSON WEB TOKEN
pub async fn login(State(state): State<AppState>, payload: LoginRequest) -> Result<LoginResponse, HttpError> {
    let user_id = match get_id_by_name_if_password(state.db, payload.name, payload.password).await.map_err(|e| e.into())? {
        Some(user_id) => user_id,
        None => { return Err(HttpError::Unauthorized("Invalid credentials".to_string())); }
    };

    let token = generate_user_jwt(user_id).map_err(|e| e.into())?;

    Ok(LoginResponse { token })
}

#[derive(Deserialize, Validate, JsonRequest)]
pub struct RegisterRequest {
    #[validate(min_length = 1)]
    name: String,
    #[validate(min_length = 1)]
    password: String
}

#[derive(Serialize, JsonResponse)]
pub struct RegisterResponse {
    token: String,
}

/// POST /user/register
///
/// registers a new user via username and password
pub async fn register(State(state): State<AppState>, payload: RegisterRequest) -> Result<RegisterResponse, HttpError> {

    let user_create_information = UserCreateInformation {
        name: payload.name,
        password: payload.password,
    };

    let user = create_user(state.db, user_create_information).await.map_err(|e| e.into())?;

    let token = generate_user_jwt(user.id).map_err(|e| e.into())?;

    Ok(RegisterResponse {
        token
    })
}

#[cfg(test)]
mod test {
    use super::*;

    mod login {
        use axum::extract::State;
        use crate::AppState;
        use crate::infrastructure::auth::jwt::generate_user_jwt_mock;
        use crate::infrastructure::db::create_test_pool;
        use crate::error::{HttpError};
        use crate::infrastructure::handler::user::{login, LoginRequest};
        use crate::infrastructure::repository::user::{get_id_by_name_if_password_mock};

        #[tokio::test]
        async fn returns_an_token() {
            generate_user_jwt_mock::setup(|i| {
                Ok(format!("token {}", i))
            });
            get_id_by_name_if_password_mock::setup(|_| {
                Ok(Some(1))
            });

            let pool = create_test_pool().await;
            let state = AppState { db: pool };

            let payload = LoginRequest {
                name: "test user".to_string(),
                password: "test password".to_string(),
            };

            let result = login(State(state), payload).await.unwrap();

            assert_eq!(result.token, "token 1".to_string());

            get_id_by_name_if_password_mock::assert_with("test user".to_string(), "test password".to_string());
        }

        #[tokio::test]
        async fn invalid_credentials() {
            get_id_by_name_if_password_mock::setup(|_| {
                Ok(None)
            });
            generate_user_jwt_mock::setup(|i| {
                Ok(format!("token {}", i))
            });

            let pool = create_test_pool().await;
            let state = AppState { db: pool };

            let payload = LoginRequest {
                name: "test user".to_string(),
                password: "test password".to_string(),
            };

            let err = login(State(state), payload).await.unwrap_err();

            assert_eq!(err, HttpError::Unauthorized("Invalid credentials".to_string()));

            generate_user_jwt_mock::assert_times(0);
        }
    }

    mod register {
        use crate::infrastructure::auth::jwt::generate_user_jwt_mock;
        use super::*;
        use crate::infrastructure::db::create_test_pool;
        use crate::entity::User;
        use crate::infrastructure::repository::user::create_user_mock;

        #[tokio::test]
        async fn returns_a_token() {
            generate_user_jwt_mock::setup(|_| {
                Ok("Mock Token".to_string())
            });
            create_user_mock::setup(|_| {
                Ok(User {
                    id: 1,
                    name: "mock user".to_string(),
                    password: "mock password".to_string(),
                })
            });

            let pool = create_test_pool().await;
            let state = AppState { db: pool };

            let payload = RegisterRequest {
                name: "test user".to_string(),
                password: "password123".to_string(),
            };

            let result = register(State(state), payload).await;

            assert!(result.is_ok());
            let response = result.unwrap();
            assert!(!response.token.is_empty());

            generate_user_jwt_mock::assert_times(1);
            generate_user_jwt_mock::assert_with(1);
        }

        #[tokio::test]
        async fn adds_user_to_the_database() {
            generate_user_jwt_mock::setup(|_| {
                Ok("Mock Token".to_string())
            });
            create_user_mock::setup(|_| {
                Ok(User {
                    id: 1,
                    name: "mock user".to_string(),
                    password: "mock password".to_string(),
                })
            });

            let pool = create_test_pool().await;
            let state = AppState { db: pool.clone() };

            let payload = RegisterRequest {
                name: "test user".to_string(),
                password: "password123".to_string(),
            };

            let result = register(State(state), payload).await;

            assert!(result.is_ok());

            create_user_mock::assert_times(1);
            create_user_mock::assert_with(UserCreateInformation {
                name: "test user".to_string(),
                password: "password123".to_string(),
            });
        }
    }
}