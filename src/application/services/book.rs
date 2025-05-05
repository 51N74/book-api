use std::sync::Arc;

use crate::domain::repositories::book::BookRepository;

pub struct BookService<T>
where T: BookRepository + Sync + Send 
{
    pub book_repositoriy:Arc<T>,
}

impl<T> BookService<T>
where T: BookRepository + Sync + Send 
{
    pub fn new(book_repositoriy: Arc<T>) -> Self {
        Self { book_repositoriy }
    }

    pub async fn create(&self, book: T::Entity) -> Result<i32, String> {
        self.book_repositoriy.create(book).await
    }
}