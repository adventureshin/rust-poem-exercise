use poem::{
    http::{header::AUTHORIZATION, StatusCode},
    test::{TestClient as PoemTestClient, TestRequestBuilder, TestResponse},
};
use sea_orm::{Database, DatabaseConnection};
use serde_json::{json, Value as JsonValue};

pub async fn setup_db() -> DatabaseConnection {
    let config = app::config::Config::from_env();
    Database::connect(&config.database_url())
        .await
        .expect("Failed to connect to database")
}

pub struct TestClient {
    client: PoemTestClient<app::App>,
    pub db: DatabaseConnection,
    token: Option<String>,
}

impl TestClient {
    pub async fn new() -> TestClient {
        let db = setup_db().await;
        let app = app::create_app(db.clone()).await;
        let client = PoemTestClient::new(app);
        TestClient {
            client,
            db,
            token: None,
        }
    }

    pub async fn sign_in(&mut self, username: &str, password: &str) {
        let resp = self
            .client
            .post("/api/auth/token")
            .body_json(&json!({"username": username, "password": password}))
            .send()
            .await;
        self.extract_token(resp).await;
    }

    async fn extract_token(&mut self, resp: TestResponse) {
        resp.assert_status(StatusCode::CREATED);
        let resp: JsonValue = resp.json().await.value().deserialize();
        self.token = Some(
            resp["result"]["token"]
                .as_str()
                .expect("Failed to extract access token.")
                .to_owned(),
        );
    }

    fn add_auth_header<'a>(
        &'a self,
        req: TestRequestBuilder<'a, app::App>,
    ) -> TestRequestBuilder<'a, app::App> {
        if self.token.is_some() {
            req.header(
                AUTHORIZATION,
                format!("Bearer {}", self.token.clone().unwrap()),
            )
        } else {
            req
        }
    }

    pub fn get(&self, uri: &str) -> TestRequestBuilder<'_, app::App> {
        let req = self.client.get(uri);
        self.add_auth_header(req)
    }

    pub fn post(&self, uri: &str, body: &JsonValue) -> TestRequestBuilder<'_, app::App> {
        let req = self.client.post(uri).body_json(body);
        self.add_auth_header(req)
    }

    pub fn patch(&self, uri: &str, body: &JsonValue) -> TestRequestBuilder<'_, app::App> {
        let req = self.client.patch(uri).body_json(body);
        self.add_auth_header(req)
    }
}
