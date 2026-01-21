use sqlx::{query_as, PgPool};

use crate::common::AppError;
use crate::scheme::{InsertUser, User};

pub struct UserRepo;

impl UserRepo {
    pub async fn get_users_list(
        pool: &PgPool,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<(Vec<User>, i64, Option<i64>), AppError> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        let users = query_as::<_, User>("SELECT * FROM user_table LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;
        let users_count = None;
        Ok((users, offset, users_count))
    }

    pub async fn get_user_by_id(pool: &PgPool, id: i64) -> Result<User, AppError> {
        let user = query_as::<_, User>("SELECT * FROM user_table WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }

    pub async fn get_user_by_username(pool: &PgPool, username: &str) -> Result<User, AppError> {
        let user = query_as::<_, User>("SELECT * FROM user_table WHERE username = $1")
            .bind(username)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }

    pub async fn insert_user(pool: &PgPool, insert_user: InsertUser) -> Result<User, AppError> {
        let user = query_as::<_, User>(
            "INSERT INTO user_table (username, password_hash, is_superuser) VALUES ($1, $2, $3) RETURNING *")
            .bind(insert_user.username)
            .bind(insert_user.password_hash)
            .bind(insert_user.is_superuser)
            .fetch_one(pool)
            .await?;
        Ok(user)
    }
}
