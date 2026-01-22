use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

use crate::common::AppError;
use crate::entity::users;
use crate::scheme::{InsertUser, User};

pub struct UserRepo;

impl UserRepo {
    pub async fn get_users_list(
        db: &DatabaseConnection,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<(Vec<User>, u64, Option<u64>), AppError> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        let users: Vec<User> = users::Entity::find()
            .order_by_asc(users::Column::Id)
            .limit(limit)
            .offset(offset)
            .all(db)
            .await?
            .into_iter()
            .map(User::from)
            .collect();
        let users_count = None;
        Ok((users, offset, users_count))
    }

    pub async fn get_user_by_id(db: &DatabaseConnection, id: i32) -> Result<User, AppError> {
        let user = users::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(AppError::ObjectNotFound)?;
        Ok(User::from(user))
    }

    pub async fn get_user_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> Result<User, AppError> {
        let user = users::Entity::find()
            .filter(users::Column::Username.eq(username))
            .order_by_desc(users::Column::Id)
            .one(db)
            .await?
            .ok_or(AppError::ObjectNotFound)?;
        Ok(User::from(user))
    }

    pub async fn insert_user(
        db: &DatabaseConnection,
        insert_user: InsertUser,
    ) -> Result<User, AppError> {
        let user_active = users::ActiveModel {
            id: Default::default(),
            username: sea_orm::Set(insert_user.username),
            password_hash: sea_orm::Set(insert_user.password_hash),
            is_super_user: sea_orm::Set(insert_user.is_superuser),
        };
        let user = user_active.insert(db).await?;
        Ok(User::from(user))
    }
}
