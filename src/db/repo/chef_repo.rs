use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, QueryOrder,
    QuerySelect,
};

use crate::common::AppError;
use crate::entity::chefs;
use crate::scheme::{Chef};

pub struct ChefRepo;

impl crate::db::repo::ChefRepo {
    pub async fn get_chefs_list(
        db: &DatabaseConnection,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<(Vec<Chef>, u64, Option<u64>), AppError> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        let users: Vec<Chef> = chefs::Entity::find()
            .order_by_desc(chefs::Column::Id)
            .limit(limit)
            .offset(offset)
            .all(db)
            .await?
            .into_iter()
            .map(Chef::from)
            .collect();
        let users_count = None;
        Ok((users, offset, users_count))
    }
    
    pub async fn get_chef_by_id(db: &DatabaseConnection, id: i32) -> Result<Chef, AppError> {
        let chef = chefs::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(AppError::ObjectNotFound)?;
        Ok(Chef::from(chef))
    }
}
