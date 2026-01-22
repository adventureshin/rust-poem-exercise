mod common;

use app::entity::users;
use poem::http::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde_json::{json, Value as JsonValue};

#[tokio::test]
async fn get_users() {
    let mut client = common::TestClient::new().await;

    client.sign_in("admin", "12345").await;

    let resp = client.get("/api/user").send().await;
    resp.assert_status(StatusCode::OK);

    let resp: JsonValue = resp.json().await.value().deserialize();
    assert_eq!(resp["status"], "success");
    assert!(resp["result"]["items"].is_array());
}

#[tokio::test]
async fn get_user_by_id() {
    let mut client = common::TestClient::new().await;

    client.sign_in("admin", "12345").await;

    // 존재하는 유저 조회
    let resp = client.get("/api/user/1").send().await;
    resp.assert_status(StatusCode::OK);

    let resp: JsonValue = resp.json().await.value().deserialize();
    assert_eq!(resp["status"], "success");
    assert_eq!(resp["result"]["id"], 1);

    // 존재하지 않는 유저 조회
    let resp = client.get("/api/user/99999").send().await;
    resp.assert_status(StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn create_user() {
    let mut client = common::TestClient::new().await;

    client.sign_in("admin", "12345").await;


    let new_username = format!("testuser_{}", chrono::Utc::now().timestamp());

    // 유저 생성
    let resp = client
        .post(
            "/api/user",
            &json!({
                "username": new_username,
                "password": "testpass123"
            }),
        )
        .send()
        .await;
    resp.assert_status(StatusCode::CREATED);

    let resp: JsonValue = resp.json().await.value().deserialize();
    assert_eq!(resp["status"], "success");
    assert_eq!(resp["result"]["username"], new_username);

    // SeaORM으로 DB에서 직접 확인
    let user = users::Entity::find()
        .filter(users::Column::Username.eq(&new_username))
        .one(&client.db)
        .await
        .unwrap();

    assert!(user.is_some());
    let user = user.unwrap();
    assert_eq!(user.username, new_username);
    assert!(!user.is_super_user);

    // 테스트 데이터 정리
    users::Entity::delete_by_id(user.id)
        .exec(&client.db)
        .await
        .unwrap();
}

#[tokio::test]
async fn unauthorized_access() {
    let client = common::TestClient::new().await;

    // 인증 없이 접근
    let resp = client.get("/api/user").send().await;
    resp.assert_status(StatusCode::UNAUTHORIZED);
}
