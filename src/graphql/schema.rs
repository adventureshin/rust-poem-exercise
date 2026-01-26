use async_graphql::{Context, Object, Result};
use poem::error::ResponseError;
use sea_orm::DatabaseConnection;
use crate::common::AppError;
use crate::db::repo::ChefRepo;
use crate::scheme::Chef;
use crate::service::auth_service::Claims;

pub struct Query;

#[Object]
impl Query {
    /// 모든 Chef 목록 조회
    async fn chefs(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Result<Vec<Chef>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let chefs = ChefRepo::get_chefs_list(db, limit, offset).await?;
        Ok(chefs.0)
    }

    /// ID로 Chef 조회
    async fn chef(&self, ctx: &Context<'_>, id: i32) -> Result<Option<Chef>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let chef = ChefRepo::get_chef_by_id(db, id).await.ok();
        Ok(chef)
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_chef(
        &self,
        ctx: &Context<'_>,
        name: String,
        short_info: String,
        description: Option<String>,
        season: i32,
        source: String,
        profile_key: Option<String>,
    ) -> Result<Chef, async_graphql::Error> {
        let db = ctx.data::<DatabaseConnection>()
            .map_err(|e| async_graphql::Error::new(e.message))?;
        let db = ctx.data::<DatabaseConnection>()?;
        let chef = ChefRepo::insert_chef(
            db,
            name,
            short_info,
            description,
            season,
            source,
            profile_key,
        ).await?;
        Ok(chef)
    }
}
