mod common;

use poem::http::StatusCode;
use serde_json::{json, Value as JsonValue};

#[tokio::test]
async fn sign_in_success() {
    let client = common::TestClient::new().await;

    let resp = client
        .post(
            "/api/auth/token",
            &json!({"username": "admin", "password": "12345"}),
        )
        .send()
        .await;
    resp.assert_status(StatusCode::CREATED);

    let resp: JsonValue = resp.json().await.value().deserialize();
    assert_eq!(resp["status"], "success");
    assert!(resp["result"]["token"].is_string());
}

#[tokio::test]
async fn sign_in_invalid_credentials() {
    let client = common::TestClient::new().await;

    let resp = client
        .post(
            "/api/auth/token",
            &json!({"username": "foo", "password": "wrong"}),
        )
        .send()
        .await;
    resp.assert_status(StatusCode::UNAUTHORIZED);

    let resp: JsonValue = resp.json().await.value().deserialize();
    assert_eq!(
        resp,
        json!({
            "status": "error",
            "reason": "Invalid credentials."
        })
    );
}

#[tokio::test]
async fn sign_in_missing_fields() {
    let client = common::TestClient::new().await;

    // username 없이
    let resp = client
        .post("/api/auth/token", &json!({"password": "12345"}))
        .send()
        .await;
    resp.assert_status(StatusCode::BAD_REQUEST);

    // password 없이
    let resp = client
        .post("/api/auth/token", &json!({"username": "admin"}))
        .send()
        .await;
    resp.assert_status(StatusCode::BAD_REQUEST);
}
