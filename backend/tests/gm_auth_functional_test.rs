use reqwest::StatusCode;
use serde_json::{Value};

use crate::common::app::spawn_app;
use crate::common::auth::gm_login_request;

mod common;

mod gm_auth_functional_test {
    use super::*;

    mod gm_auth {
        use super::*;

        #[tokio::test]
        async fn gm_login_with_correct_password_returns_token() {
            let addr = spawn_app().await;

            let resp = gm_login_request(&addr, "test-password").await;

            assert_eq!(resp.status(), StatusCode::OK);
            let body: Value = resp.json().await.unwrap();
            assert!(body["token"].is_string());
            assert!(!body["token"].as_str().unwrap().is_empty());
        }

        #[tokio::test]
        async fn gm_login_with_wrong_password_returns_unauthorized() {
            let addr = spawn_app().await;

            let resp = gm_login_request(&addr, "wrong-password").await;

            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
            let body: Value = resp.json().await.unwrap();
            assert!(body["error"].is_string());
        }
    }

    mod protected_routes {
        use crate::common::app::spawn_app;
        use crate::common::auth::gm_login;
        use super::*;

        #[tokio::test]
        async fn protected_route_without_token_returns_unauthorized() {
            let addr = spawn_app().await;

            let client = reqwest::Client::new();
            let resp = client
                .get(format!("http://{addr}/gm/games"))
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
                .get(format!("http://{addr}/gm/games"))
                .header("Authorization", "Bearer invalid-token")
                .send()
                .await
                .unwrap();

            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        }

        #[tokio::test]
        async fn protected_route_with_valid_token_returns_ok() {
            let addr = spawn_app().await;
            let token = gm_login(&addr).await.unwrap().token;

            let client = reqwest::Client::new();
            let resp = client
                .get(format!("http://{addr}/gm/games"))
                .header("Authorization", format!("Bearer {token}"))
                .send()
                .await
                .unwrap();

            assert_eq!(resp.status(), StatusCode::OK);
            let body: Value = resp.json().await.unwrap();
            assert!(body["games"].is_array());
        }
    }
}