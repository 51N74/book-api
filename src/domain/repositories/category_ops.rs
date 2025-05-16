use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::category::{AddCategoryEntity, EditCategoryEntity};



#[async_trait]
#[automock]
pub trait CategoryOpsRepository {
    async fn add(&self, add_category_entity: AddCategoryEntity) -> Result<i32>;
    async fn edit(&self, category_id: i32, edit_category_entity: EditCategoryEntity) -> Result<i32>;
    async fn remove(&self, category_id: i32,admin_id: i32) -> Result<()>;
}
