use axum::async_trait;
use mockall::automock;
use uuid::Uuid;


use anyhow::Result;

use crate::domain::entities::category:: CategoryEntity;

#[async_trait]
#[automock]
pub trait CategoryRepository {
    async fn create(&self, category: CategoryEntity) -> Result<i32>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CategoryEntity>, String>;
    async fn find_all(&self, query: Option<String>) -> Result<Vec<CategoryEntity>, String>;
    async fn update(&self, category: CategoryEntity) -> Result<CategoryEntity, String>;
    async fn delete(&self, id: Uuid) -> Result<(), String>;
}
