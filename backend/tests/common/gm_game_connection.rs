use std::net::SocketAddr;
use axum::http::StatusCode;
use serde_json::{json, Value};

pub async fn create_game(addr: &SocketAddr, token: &str, name: &str) -> String {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://{addr}/gm/games"))
        .header("Authorization", format!("Bearer {token}"))
        .header("Content-Type", "application/json")
        .json(&json!({ "name": name }))
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = resp.json().await.unwrap();
    body["id"].as_str().unwrap().to_string()
}