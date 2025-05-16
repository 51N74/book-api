use std::sync::Arc;

use anyhow::Result;

use crate::domain::{repositories::{book_viewing::BookViewingRepository, switchboard::SwitchboardRepository, transaction_provider::TransactionProvider}, value_objects::{book_statuses::BookStatuses, book_user_junction::{BookUserJunction, MAX_ADVENTURERS_PER_QUEST}}};

pub struct SwitchboardService<T1, T2, T3>
where
    T1: SwitchboardRepository + Send + Sync,
    T2: BookViewingRepository + Send + Sync,
    T3: TransactionProvider + Send + Sync,
{
    switchboard_repository: Arc<T1>,
    book_viewing_repository: Arc<T2>,
    tx: Arc<T3>,
}

impl<T1, T2, T3> SwitchboardService<T1, T2, T3>
where
    T1: SwitchboardRepository + Send + Sync + 'static,
    T2: BookViewingRepository + Send + Sync,
    T3: TransactionProvider + Send + Sync,
{
    pub fn new(
        switchboard_repository: Arc<T1>,
        book_viewing_repository: Arc<T2>,
        tx: Arc<T3>,
    ) -> Self {
        Self {
            switchboard_repository,
            book_viewing_repository,
            tx,
        }
    }

    pub async fn available(&self, book_id: i32, user_id: i32) -> Result<()> {
        let book = self.book_viewing_repository.view_details(book_id).await?;

        let user_count = self
            .book_viewing_repository
            .user_counting_by_book_id(book_id)
            .await?;

        let book_status_condition = book.status == BookStatuses::Available.to_string()
            || book.status == BookStatuses::Reserved.to_string();
        let user_count_condition = user_count < MAX_ADVENTURERS_PER_QUEST;

        if !book_status_condition {
            return Err(anyhow::anyhow!("Quest is not joinable"));
        }

        if !user_count_condition {
            return Err(anyhow::anyhow!("Quest is full"));
        }

        self.switchboard_repository
            .available(BookUserJunction {
                book_id,
                user_id,
            })
            .await?;

        Ok(())
    }

    pub async fn reserved(&self, book_id: i32, user_id: i32) -> Result<()> {
        let book = self.book_viewing_repository.view_details(book_id).await?;

        let reserved_coditions = book.status == BookStatuses::Available.to_string()
            || book.status == BookStatuses::Reserved.to_string();

        if !reserved_coditions {
            return Err(anyhow::anyhow!("Quest is not leavable"));
        }

        self.switchboard_repository
            .reserved(BookUserJunction {
                book_id,
                user_id,
            })
            .await?;

        Ok(())
    }

    pub async fn available_and_reserved_transaction(
        &self,
        book_id: i32,
        user_id: i32,
    ) -> Result<()> {
        let tx = Arc::clone(&self.tx);
        let repo = Arc::clone(&self.switchboard_repository);

        tx.transaction::<_, anyhow::Error, _>(move |conn| {
            repo.for_transaction_test_1(
                conn,
                BookUserJunction {
                    book_id,
                    user_id,
                },
            )?;

            repo.for_transaction_test_2(
                conn,
                BookUserJunction {
                  book_id,
                    user_id,
                },
            )?;

            Ok(())
        })
    }
}