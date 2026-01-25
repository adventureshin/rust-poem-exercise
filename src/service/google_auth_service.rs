use poem_openapi::Object;
use serde::Deserialize;
use crate::config::Config;

#[derive(Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub scope: String,
    pub id_token: Option<String>,
}

#[derive(Deserialize, Debug, Object)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub picture: Option<String>,
}

pub struct GoogleAuthService;

impl GoogleAuthService {
    /// Google OAuth 로그인 URL 생성
    pub fn build_auth_url(config: &Config) -> String {
        let scope = "openid%20email%20profile";
        let redirect_uri_encoded = config.google_redirect_url
            .replace(":", "%3A")
            .replace("/", "%2F");
        format!(
            "https://accounts.google.com/o/oauth2/v2/auth\
             ?client_id={}\
             &redirect_uri={}\
             &response_type=code\
             &scope={}",
            config.google_client_id,
            redirect_uri_encoded,
            scope
        )
    }

    /// Authorization code를 access token으로 교환
    pub async fn exchange_code_for_token(
        config: &Config,
        code: &str,
    ) -> Result<GoogleTokenResponse, reqwest::Error> {
        let client = reqwest::Client::new();
        let params = [
            ("code", code),
            ("client_id", &config.google_client_id),
            ("client_secret", &config.google_client_secret),
            ("redirect_uri", &config.google_redirect_url),
            ("grant_type", "authorization_code"),
        ];

        let response = client
            .post("https://oauth2.googleapis.com/token")
            .form(&params)
            .send()
            .await?
            .json::<GoogleTokenResponse>()
            .await?;

        Ok(response)
    }

    /// Access token으로 Google 사용자 정보 가져오기
    pub async fn get_user_info(access_token: &str) -> Result<GoogleUserInfo, reqwest::Error> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await?
            .json::<GoogleUserInfo>()
            .await?;

        Ok(response)
    }
}
