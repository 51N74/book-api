
use uuid::Uuid;

use crate::domain::{entities::book::Book, repositories::repositories::BookRepository};

pub struct BookService<T: BookRepository> {
    repository: T,
}

impl<T: BookRepository> BookService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn add_book(&self, book: Book) -> Result<Book, String> {
        self.repository.create(book)
    }

    pub fn get_book(&self, id: Uuid) -> Result<Option<Book>, String> {
        self.repository.find_by_id(id)
    }

    pub fn search_books(&self, query: Option<String>) -> Result<Vec<Book>, String> {
        self.repository.find_all(query)
    }

    pub fn update_book(&self, book: Book) -> Result<Book, String> {
        self.repository.update(book)
    }

    pub fn delete_book(&self, id: Uuid) -> Result<(), String> {
        self.repository.delete(id)
    }

}