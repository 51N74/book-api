use axum::async_trait;
use mockall::automock;
use uuid::Uuid;

use crate::domain::entities::book::BookEntity;
use anyhow::Result;

#[async_trait]
#[automock]
pub trait BookRepository {
    async fn create(&self, book: BookEntity) -> Result<i32>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<BookEntity>, String>;
    async fn find_all(&self, query: Option<String>) -> Result<Vec<BookEntity>, String>;
    async fn update(&self, book: BookEntity) -> Result<BookEntity, String>;
    async fn delete(&self, id: Uuid) -> Result<(), String>;
}
