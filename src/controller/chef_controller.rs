use poem::web::Data;
use poem_openapi::{
    param::{Path, Query},
    payload::Json,
    OpenApi,
};

use super::Tag;
use crate::response::{
    GetListResponseError, GetListResponseSuccess, GetResponseError, GetResponseSuccess,
    PostResponseError, PostResponseSuccess,
};
use crate::scheme::Chef;
use crate::{context::AppContext, db::repo::ChefRepo};

pub struct ChefController;


#[OpenApi(prefix_path = "/chef", tag = "Tag::Chef")]
impl ChefController {
    /// Get chefs list.
    #[oai(path = "/", method = "get")]
    async fn get_chefs_list(
        &self,
        ctx: Data<&AppContext>,
        limit: Query<Option<u64>>,
        offset: Query<Option<u64>>,
    ) -> Result<GetListResponseSuccess<Chef>, GetListResponseError> {
        let chefs = ChefRepo::get_chefs_list(&ctx.db, limit.0, offset.0).await?;
        let resp = GetListResponseSuccess::new(chefs.0, chefs.1, chefs.2);
        Ok(resp)
    }

    /// Get chef by ID.
    #[oai(path = "/:id", method = "get")]
    async fn get_chef_by_id(
        &self,
        ctx: Data<&AppContext>,
        id: Path<i32>,
    ) -> Result<GetResponseSuccess<Chef>, GetResponseError> {
        let chef = ChefRepo::get_chef_by_id(&ctx.db, id.0).await?;
        let resp = GetResponseSuccess::new(chef);
        Ok(resp)
    }
}