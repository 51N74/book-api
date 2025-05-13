use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::{entities::category::CategoryEntity, value_objects::category_viewing_filter::CategoryViewingFilter};


#[async_trait]
#[automock]
pub trait CategoryViewingRepository {
    async fn view_details(&self, category_id: i32) -> Result<CategoryEntity>;
    async fn category_viewing(&self, filter: &CategoryViewingFilter) -> Result<Vec<CategoryEntity>>;
    async fn book_counting_by_category_id(&self, category_id: i32) -> Result<i64>;
}
