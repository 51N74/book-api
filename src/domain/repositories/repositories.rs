use uuid::Uuid;

use crate::domain::entities::{book::Book, category::Category, user::User};

pub trait BookRepository {
    fn create(&self, book: Book) -> Result<Book, String>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<Book>, String>;
    fn find_all(&self, query: Option<String>) -> Result<Vec<Book>, String>;
    fn update(&self, book: Book) -> Result<Book, String>;
    fn delete(&self, id: Uuid) -> Result<(), String>;
}

pub trait UserRepository{
    fn create(&self,user:User)->Result<User, String>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<User>, String>;
    fn find_all(&self, query: Option<String>) -> Result<Vec<User>, String>;
    fn update(&self,user:User)->Result<User, String>;
    fn delete(&self,id:Uuid)->Result<(), String>;
    
}

pub trait CategoryRepository{
    fn create(&self,category:Category)->Result<Category, String>;
    fn find_by_id(&self, id: Uuid) -> Result<Option<Category>, String>;
    fn find_all(&self, query: Option<String>) -> Result<Vec<Category>, String>;
    fn update(&self,category:Category)->Result<Category, String>;
    fn delete(&self,id:Uuid)->Result<(), String>;

}