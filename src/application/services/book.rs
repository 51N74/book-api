use std::sync::Arc;
use anyhow::Result;
use crate::domain::{repositories::book::BookRepository, value_objects::book_model::RegisterBookModel};

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

    pub async fn create(&self,mut register_book_model:RegisterBookModel ) ->Result<i32> {
        unimplemented!()
    }
}