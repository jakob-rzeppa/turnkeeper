use std::net::SocketAddr;
use axum::http::StatusCode;
use serde_json::{json, Value};
use turnkeeper_backend::infrastructure::http::gm;
use crate::common::app::TEST_GM_PASSWORD;

pub async fn gm_login_request(addr: &SocketAddr, password: &str) -> reqwest::Response {
    let client = reqwest::Client::new();
    client
        .post(format!("http://{addr}/gm/login"))
        .header("Content-Type", "application/json")
        .json(&json!({ "password": password }))
        .send()
        .await
        .expect("Failed to send gm login request")
}

pub async fn gm_login(addr: &SocketAddr) -> Result<gm::LoginResponse, String> {
    let resp = gm_login_request(addr, TEST_GM_PASSWORD).await;

    if resp.status() != StatusCode::OK {
        return Err(format!("Gm login failed with status: {}", resp.status()));
    }

    let body: Value = resp.json().await.map_err(|e| format!("Failed to parse gm login response: {}", e))?;
    let token = body["token"].as_str().ok_or("Token not found in gm login response")?.to_string();

    Ok(gm::LoginResponse {
        token
    })
}

pub async fn user_register_request(addr: &SocketAddr, name: &str, password: &str) -> reqwest::Response {
    let client = reqwest::Client::new();
    client
        .post(format!("http://{addr}/user/register"))
        .header("Content-Type", "application/json")
        .json(&json!({ "name": name, "password": password }))
        .send()
        .await
        .expect("Failed to send user register request")
}

#[allow(dead_code)]
pub async fn user_login_request(addr: &SocketAddr, name: &str, password: &str) -> reqwest::Response {
    let client = reqwest::Client::new();
    client
        .post(format!("http://{addr}/user/login"))
        .header("Content-Type", "application/json")
        .json(&json!({ "name": name, "password": password }))
        .send()
        .await
        .expect("Failed to send user login request")
}

#[allow(dead_code)]
pub async fn user_register(addr: &SocketAddr, name: &str, password: &str) -> Result<gm::LoginResponse, String> {
    let resp = user_register_request(addr, name, password).await;

    if resp.status() != StatusCode::OK {
        return Err(format!("User register failed with status: {}", resp.status()));
    }

    let body: Value = resp.json().await.map_err(|e| format!("Failed to parse user register response: {}", e))?;
    let token = body["token"].as_str().ok_or("Token not found in user register response")?.to_string();

    Ok(gm::LoginResponse {
        token
    })
}