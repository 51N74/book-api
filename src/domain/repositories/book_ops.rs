use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::book::{AddBookEntity, EditBookEntity};

#[async_trait]
#[automock]
pub trait BookOpsRepository {
    async fn add(&self, add_book_entity: AddBookEntity) -> Result<i32>;
    async fn edit(&self, quest_id: i32, edit_book_entity: EditBookEntity) -> Result<i32>;
    async fn remove(&self, book_id: i32, admin_id: i32) -> Result<()>;
}