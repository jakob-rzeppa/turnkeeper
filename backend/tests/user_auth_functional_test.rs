use reqwest::StatusCode;
use serde_json::{Value};

use crate::common::app::spawn_app;
use crate::common::auth::{user_register_request, user_register};

mod common;

mod user_auth_functional_test {
    use super::*;

    mod user_registration {
        use super::*;

        #[tokio::test]
        async fn register_with_valid_credentials_returns_token() {
            let addr = spawn_app().await;

            let resp = user_register_request(&addr, "alice", "test-pass").await;

            assert_eq!(resp.status(), StatusCode::OK);
            let body: Value = resp.json().await.unwrap();
            assert!(body["token"].is_string());
            assert!(!body["token"].as_str().unwrap().is_empty());
        }

        #[tokio::test]
        async fn register_with_empty_name_returns_bad_request() {
            let addr = spawn_app().await;

            let resp = user_register_request(&addr, "", "test-pass").await;

            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
            let body: Value = resp.json().await.unwrap();
            assert!(body["error"].is_string());
        }

        #[tokio::test]
        async fn register_with_short_password_returns_bad_request() {
            let addr = spawn_app().await;

            let resp = user_register_request(&addr, "alice", "ab").await;

            assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
            let body: Value = resp.json().await.unwrap();
            assert!(body["error"].is_string());
        }

        #[tokio::test]
        async fn register_duplicate_user_returns_conflict() {
            let addr = spawn_app().await;

            let resp1 = user_register_request(&addr, "alice", "test-pass").await;
            assert_eq!(resp1.status(), StatusCode::OK);

            let resp2 = user_register_request(&addr, "alice", "test-pass").await;
            assert_eq!(resp2.status(), StatusCode::CONFLICT);
        }
    }

    mod user_login_tests {
        use crate::common::auth::user_login_request;
        use super::*;

        #[tokio::test]
        async fn login_with_correct_credentials_returns_token() {
            let addr = spawn_app().await;

            // Register first
            user_register(&addr, "bob", "test-pass").await.unwrap();

            // Now login
            let resp = user_login_request(&addr, "bob", "test-pass").await;

            assert_eq!(resp.status(), StatusCode::OK);
            let body: Value = resp.json().await.unwrap();
            assert!(body["token"].is_string());
            assert!(!body["token"].as_str().unwrap().is_empty());
        }

        #[tokio::test]
        async fn login_with_wrong_password_returns_unauthorized() {
            let addr = spawn_app().await;

            user_register(&addr, "bob", "test-pass").await.unwrap();

            let resp = user_login_request(&addr, "bob", "wrong-password").await;

            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
            let body: Value = resp.json().await.unwrap();
            assert!(body["error"].is_string());
        }

        #[tokio::test]
        async fn login_with_nonexistent_user_returns_not_found() {
            let addr = spawn_app().await;

            let resp = user_login_request(&addr, "nobody", "test-pass").await;

            assert_eq!(resp.status(), StatusCode::NOT_FOUND);
            let body: Value = resp.json().await.unwrap();
            assert!(body["error"].is_string());
        }
    }

    // ─────────────────────────────────────────────────────────────
    //  Protected Routes – User Authorization
    // ─────────────────────────────────────────────────────────────

    mod protected_routes {
        use crate::common::auth::gm_login;
        use super::*;

        #[tokio::test]
        async fn protected_route_without_token_returns_unauthorized() {
            let addr = spawn_app().await;

            let client = reqwest::Client::new();
            let resp = client
                .get(format!("http://{addr}/user/games"))
                .send()
                .await
                .unwrap();

            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        }

        #[tokio::test]
        async fn protected_route_with_invalid_token_returns_unauthorized() {
            let addr = spawn_app().await;

            let client = reqwest::Client::new();
            let resp = client
                .get(format!("http://{addr}/user/games"))
                .header("Authorization", "Bearer invalid-token")
                .send()
                .await
                .unwrap();

            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        }

        #[tokio::test]
        async fn protected_route_with_valid_token_returns_ok() {
            let addr = spawn_app().await;
            let token = user_register(&addr, "charlie", "test").await.unwrap().token;

            let client = reqwest::Client::new();
            let resp = client
                .get(format!("http://{addr}/user/games"))
                .header("Authorization", format!("Bearer {token}"))
                .send()
                .await
                .unwrap();

            assert_eq!(resp.status(), StatusCode::OK);
            let body: Value = resp.json().await.unwrap();
            assert!(body["games"].is_array());
        }

        #[tokio::test]
        async fn gm_token_cannot_access_user_routes() {
            let addr = spawn_app().await;
            let token = gm_login(&addr).await.unwrap().token;

            let client = reqwest::Client::new();
            let resp = client
                .get(format!("http://{addr}/user/games"))
                .header("Authorization", format!("Bearer {token}"))
                .send()
                .await
                .unwrap();

            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        }
    }
}