use std::sync::Arc;

use anyhow::Result;

use crate::domain::{repositories::{book_ledger::BookLedgerRepository, book_viewing::BookViewingRepository}, value_objects::{book_statuses::BookStatuses, book_user_junction::MAX_ADVENTURERS_PER_QUEST}};



pub struct BookLedgerService<T1, T2>
where
    T1: BookLedgerRepository + Send + Sync,
    T2: BookViewingRepository + Send + Sync,
{
    journey_ledger_repository: Arc<T1>,
    book_viewing_repository: Arc<T2>,
}

impl<T1, T2> BookLedgerService<T1, T2>
where
    T1: BookLedgerRepository + Send + Sync,
    T2: BookViewingRepository + Send + Sync,
{
    pub fn new(journey_ledger_repository: Arc<T1>, book_viewing_repository: Arc<T2>) -> Self {
        Self {
            journey_ledger_repository,
            book_viewing_repository,
        }
    }

    pub async fn in_borrowing(&self, book_id: i32, admin_id: i32) -> Result<i32> {
        let book = self.book_viewing_repository.view_details(book_id).await?;

        let user_number = self
            .book_viewing_repository
            .user_counting_by_book_id(book_id)
            .await?;

        let conditions_to_update = (book.status == BookStatuses::Available.to_string()
            || book.status == BookStatuses::Reserved.to_string())
            && user_number > 0
            && user_number <= MAX_ADVENTURERS_PER_QUEST;

        if !conditions_to_update {
            return Err(anyhow::anyhow!("Invalid condition to change status"));
        }

        let result = self
            .journey_ledger_repository
            .in_borrowing(book_id, admin_id)
            .await?;

        Ok(result)
    }

    pub async fn to_reserved(&self, book_id: i32, admin_id: i32) -> Result<i32> {
        let book = self.book_viewing_repository.view_details(book_id).await?;

        let conditions_to_update = book.status == BookStatuses::Reserved.to_string();

        if !conditions_to_update {
            return Err(anyhow::anyhow!("Invalid condition to change status"));
        }

        let result = self
            .journey_ledger_repository
            .to_reserved(book_id, admin_id)
            .await?;

        Ok(result)
    }

    
}