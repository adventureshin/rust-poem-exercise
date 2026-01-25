mod claims;

pub use claims::{superuser_scope, Claims};

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::DatabaseConnection;

use crate::common::AppError;
use crate::db::repo::UserRepo;
use crate::scheme::{AccessToken, LoginResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RefreshClaims {
    pub user_id: i32,
    pub is_refresh: bool,
    pub iat: i64,
    pub exp: i64,
}

pub struct AuthService;

impl AuthService {
    pub fn create_access_token(user_id: i32, is_super_user: bool) -> Result<AccessToken, AppError> {
        let iat = Utc::now();
        let exp = iat + Duration::seconds(3600);
        let iat = iat.timestamp_millis();
        let exp = exp.timestamp_millis();
        let claims = Claims {
            user_id,
            is_super_user,
            iat,
            exp,
        };
        let key = EncodingKey::from_secret("secret".as_ref());
        let header = Header::new(Algorithm::HS256);
        let token = encode(&header, &claims, &key)?;
        Ok(AccessToken {
            token,
            token_type: "Bearer".to_string(),
            issued_at: iat,
            expired_in: exp,
        })
    }

    pub async fn create_refresh_token(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<String, AppError> {
        let user = UserRepo::get_user_by_id(db, user_id).await?;
        let iat = Utc::now();
        let exp = iat + Duration::days(7);
        let iat = iat.timestamp_millis();
        let exp = exp.timestamp_millis();
        let claims = RefreshClaims {
            user_id: user.id,
            is_refresh: true,
            iat,
            exp,
        };
        let key = EncodingKey::from_secret("secret".as_ref());
        let header = Header::new(Algorithm::HS256);
        let token = encode(&header, &claims, &key)?;
        Ok(token)
    }

    pub fn verify_access_token(token: &str) -> Result<Claims, AppError> {
        let key = DecodingKey::from_secret("secret".as_ref());
        let validation = Validation::new(Algorithm::HS256);
        let data = decode::<Claims>(token, &key, &validation)?;
        let claims = data.claims;
        let now = Utc::now();
        if now.timestamp_millis() > claims.exp {
            return Err(AppError::AccessTokenExpired);
        }
        Ok(claims)
    }

    pub fn verify_refresh_token(token: &str) -> Result<RefreshClaims, AppError> {
        let key = DecodingKey::from_secret("secret".as_ref());
        let validation = Validation::new(Algorithm::HS256);
        let data = decode::<RefreshClaims>(token, &key, &validation)?;
        let claims = data.claims;
        let now = Utc::now();
        if now.timestamp_millis() > claims.exp {
            return Err(AppError::RefreshTokenExpired);
        }
        Ok(claims)
    }

    pub async fn create_login_response(
        db: &DatabaseConnection,
        user_id: i32,
        is_super_user: bool,
    ) -> Result<LoginResponse, AppError> {
        let access_token = AuthService::create_access_token(user_id, is_super_user)?;
        let refresh_token = AuthService::create_refresh_token(db, user_id).await?;
        Ok(LoginResponse {
            access_token,
            refresh_token,
        })
    }

    pub fn hash_password(pwd: &str) -> Result<String, AppError> {
        let hash = bcrypt::hash(pwd, bcrypt::DEFAULT_COST)?;
        Ok(hash)
    }

    pub fn is_valid_password(pwd: &str, pwd_hash: &str) -> bool {
        bcrypt::verify(pwd, pwd_hash).unwrap_or(false)
    }

    // pub async fn sign_in(
    //     db: &DatabaseConnection,
    //     credentials: Credentials,
    // ) -> Result<AccessToken, AppError> {
    //     match UserRepo::get_user_by_username(db, &credentials.username).await {
    //         Ok(user) => {
    //             if AuthService::is_valid_password(&credentials.password, &user.password_hash) {
    //                 let token = AuthService::create_access_token(user.id, user.is_super_user)?;
    //                 return Ok(token);
    //             }
    //             Err(AppError::InvalidCredentials)
    //         }
    //         Err(e) => match e {
    //             AppError::ObjectNotFound => Err(AppError::InvalidCredentials),
    //             _ => Err(e),
    //         },
    //     }
    // }
}
