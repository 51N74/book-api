use anyhow::Result;
use axum::async_trait;
use mockall::automock;

#[async_trait]
#[automock]
pub trait BookLedgerRepository {
    async fn in_borrowing(&self, book_id: i32, admin_id: i32) -> Result<i32>;
    async fn to_reserved(&self, book_id: i32, admin_id: i32) -> Result<i32>;
}