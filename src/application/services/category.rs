
use uuid::Uuid;

use crate::{domain::{entities::category::Category, repositories::repositories::CategoryRepository}, infrastructure::postgres::schema::categories};



pub struct CategoryService<T: CategoryRepository> {
    repository: T,
}

impl<T: CategoryRepository> CategoryService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn add_category(&self, category: Category) -> Result<Category, String> {
        self.repository.create(category)
    }

    pub fn get_category(&self, id: Uuid) -> Result<Option<Category>, String> {
        self.repository.find_by_id(id)
    }

    pub fn search_categorys(&self, query: Option<String>) -> Result<Vec<Category>, String> {
        self.repository.find_all(query)
    }

    pub fn update_category(&self, category: Category) -> Result<Category, String> {
        self.repository.update(category)
    }

    pub fn delete_category(&self, id: Uuid) -> Result<(), String> {
        self.repository.delete(id)
    }

}