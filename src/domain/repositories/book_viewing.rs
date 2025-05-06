use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::{
    entities::book::BookEntity, value_objects::book_viewing_filter::BookViewingFilter,
};

#[async_trait]
#[automock]
pub trait BookViewingRepository {
    async fn view_details(&self, book_id: i32) -> Result<BookEntity>;
    async fn book_viewing(&self, filter: &BookViewingFilter) -> Result<Vec<BookEntity>>;
    async fn user_counting_by_book_id(&self, book_id: i32) -> Result<i64>;
}