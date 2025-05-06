use std::sync::Arc;

use anyhow::Result;

use crate::domain::{repositories::book_viewing::BookViewingRepository, value_objects::{book_model::{self, BookModel}, book_viewing_filter::BookViewingFilter}};





pub struct BookViewingService<T>
where
    T: BookViewingRepository + Send + Sync,
{
    book_viewing_repository: Arc<T>,
}

impl<T> BookViewingService<T>
where
    T: BookViewingRepository + Send + Sync,
{
    pub fn new(book_viewing_repository: Arc<T>) -> Self {
        Self {
            book_viewing_repository,
        }
    }

    pub async fn view_details(&self, book_id: i32) -> Result<BookModel> {
        let result = self.book_viewing_repository.view_details(book_id).await?;

        let user_count = self
            .book_viewing_repository
            .user_counting_by_book_id(result.id)
            .await?;

        let book_model = result.to_model(user_count.try_into().unwrap());

        Ok(book_model)
    }

    pub async fn book_viewing(&self, filter: &BookViewingFilter) -> Result<Vec<BookModel>> {
        let results = self.book_viewing_repository.book_viewing(filter).await?;

        let mut book_model = Vec::new();

        for result in results.into_iter() {
            let user_count = self
                .book_viewing_repository
                .user_counting_by_book_id(result.id)
                .await
                .unwrap_or(0);

            book_model.push(result.to_model(user_count.try_into().unwrap()));
        }

        Ok(book_model)
    }
}