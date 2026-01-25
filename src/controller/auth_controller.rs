use poem::web::Data;
use poem_openapi::{payload::Json, param::Query, ApiResponse, OpenApi};

use super::Tag;
use crate::context::AppContext;
use crate::db::repo::UserRepo;
use crate::response::{PostResponseError, PostResponseSuccess};
use crate::scheme::{AccessToken, InsertUser, LoginResponse};
use crate::service::auth_service::AuthService;
use crate::service::google_auth_service::{GoogleAuthService, GoogleUserInfo};

#[derive(ApiResponse)]
pub enum GoogleLoginResponse {
    #[oai(status = 302)]
    Redirect(#[oai(header = "Location")] String),
}

pub struct AuthController;

#[OpenApi(prefix_path = "/auth", tag = "Tag::Auth")]
impl AuthController {
    /// Google 로그인 시작 - Google OAuth 페이지로 리다이렉트
    #[oai(path = "/google/login", method = "get")]
    async fn google_login(&self, ctx: Data<&AppContext>) -> GoogleLoginResponse {
        let url = GoogleAuthService::build_auth_url(&ctx.config);
        GoogleLoginResponse::Redirect(url)
    }

    /// Google OAuth Callback - 토큰 교환 및 사용자 정보 반환
    #[oai(path = "/google/callback", method = "get")]
    async fn google_callback(
        &self,
        ctx: Data<&AppContext>,
        code: Query<String>,
    ) -> Result<PostResponseSuccess<LoginResponse>, PostResponseError> {
        // 1. code로 access_token 교환
        let token_response = GoogleAuthService::exchange_code_for_token(&ctx.config, &code.0)
            .await
            .map_err(|e| {
                let err = crate::common::AppError::UnhandledError(e.to_string());
                PostResponseError::from(err)
            })?;

        // 2. access_token으로 사용자 정보 가져오기
        let google_user = GoogleAuthService::get_user_info(&token_response.access_token)
            .await
            .map_err(|e| {
                let err = crate::common::AppError::UnhandledError(e.to_string());
                PostResponseError::from(err)
            })?;

        let user = UserRepo::insert_user(
            &ctx.db,
            InsertUser {
                name: google_user.email.clone(),
                email: google_user.email,
                google_id: google_user.id,
                is_super_user: false,
                profile_url: google_user.picture,
            },
        ).await?;
        Ok(PostResponseSuccess::new(AuthService::create_login_response(
            &ctx.db,
            user.id,
            user.is_super_user,
        ).await?))
    }
}
