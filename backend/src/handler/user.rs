use axum::extract::{State};
use fnmock::derive::use_mock;
use crate::error::{HttpError};
use crate::{get_db_connection, json_handler, AppState};
use crate::repository::user::{create_user, UserCreateInformation};

#[use_mock]
use crate::auth::jwt::generate_user_jwt;

json_handler!(Login, {
    name: String,
    password: String
}, {
    token: String,
});

/// POST /user/login
///
/// authenticates a user via username and password and returns a JSON WEB TOKEN
pub async fn login(State(state): State<AppState>, payload: LoginRequest) -> Result<LoginResponse, HttpError> {
    Err(HttpError::NotImplemented)
}

json_handler!(Register, {
    name: String,
    password: String
}, {
    token: String,
});

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

    mod register {
        use crate::auth::jwt::generate_user_jwt_mock;
        use super::*;
        use crate::db::create_test_pool;

        #[tokio::test]
        async fn returns_a_token() {
            generate_user_jwt_mock::setup(|_| {
                Ok("Mock Token".to_string())
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
            let pool = create_test_pool().await;
            let state = AppState { db: pool.clone() };

            let payload = RegisterRequest {
                name: "test user".to_string(),
                password: "password123".to_string(),
            };

            let result = register(State(state), payload).await;

            assert!(result.is_ok());

            let mut conn = pool.acquire().await.unwrap();
            let rows = sqlx::query!("SELECT * FROM users").fetch_all(&mut *conn).await.unwrap();

            assert_eq!(1, rows.len());
            assert_eq!("test user", rows[0].name);
            assert_eq!("password123", rows[0].password);
        }
    }
}