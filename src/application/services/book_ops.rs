use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    repositories::{book_ops::BookOpsRepository, book_viewing::BookViewingRepository},
    value_objects::book_model::{AddBookModel, EditBookModel},
};

pub struct BookOpsService<T1, T2>
where
    T1: BookOpsRepository + Send + Sync,
    T2: BookViewingRepository + Send + Sync,
{
    book_ops_repository: Arc<T1>,
    book_viewing_repository: Arc<T2>,
}

impl<T1, T2> BookOpsService<T1, T2>
where
    T1: BookOpsRepository + Send + Sync,
    T2: BookViewingRepository + Send + Sync,
{
    pub fn new(book_ops_repository: Arc<T1>, book_viewing_repository: Arc<T2>) -> Self {
        Self {
            book_ops_repository,
            book_viewing_repository,
        }
    }

    pub async fn add(
        &self,
        admin_id: i32,
        add_book_model: AddBookModel,
    ) -> Result<i32> {
        let insert_book_entity = add_book_model.to_entity(admin_id);

        let result = self.book_ops_repository.add(insert_book_entity).await?;

        Ok(result)
    }

    pub async fn edit(
        &self,
        book_id: i32,
        admin_id: i32,
        edit_book_model: EditBookModel,
    ) -> Result<i32> {
        // Check if adventurer exists
        let user_count = self
            .book_viewing_repository
            .user_counting_by_book_id(book_id)
            .await?;

        if user_count > 0 {
            return Err(anyhow::anyhow!(
                "Quest has been taken by adventurers for now!!!"
            ));
        }

        // Update quest
        let edit_book_entity = edit_book_model.to_entity(admin_id);

        let result = self
            .book_ops_repository
            .edit(book_id, edit_book_entity)
            .await?;

        Ok(result)
    }

    pub async fn remove(&self, book_id: i32, admin_id: i32) -> Result<()> {
        // Check if adventurer exists
        let user_count = self
            .book_viewing_repository
            .user_counting_by_book_id(book_id)
            .await?;

        if user_count > 0 {
            return Err(anyhow::anyhow!(
                "Quest has been taken by adventurers for now!!!"
            ));
        }

        self.book_ops_repository
            .remove(book_id, admin_id)
            .await?;

        Ok(())
    }
}