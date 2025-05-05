use axum::async_trait;
use mockall::automock;
use uuid::Uuid;

use crate::domain::entities::user::UserEntity;
use anyhow::Result;

#[async_trait]
#[automock]
pub trait UserRepository {
    async fn create(&self, user: UserEntity) -> Result<i32>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserEntity>, String>;
    async fn find_all(&self, query: Option<String>) -> Result<Vec<UserEntity>, String>;
    async fn update(&self, user: UserEntity) -> Result<UserEntity, String>;
    async fn delete(&self, id: Uuid) -> Result<(), String>;
}
