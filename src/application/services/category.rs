
use uuid::Uuid;

use crate::{domain::{entities::category::CategoryEntity, repositories::category::CategoryRepository}, infrastructure::postgres::schema::categories};



pub struct CategoryService<T: CategoryRepository> {
    repository: T,
}

impl<T: CategoryRepository> CategoryService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn add_category(&self, category: CategoryEntity) -> Result<CategoryEntity, String> {
        self.repository.create(category)
    }

    pub fn get_category(&self, id: Uuid) -> Result<Option<CategoryEntity>, String> {
        self.repository.find_by_id(id)
    }

    pub fn search_categorys(&self, query: Option<String>) -> Result<Vec<CategoryEntity>, String> {
        self.repository.find_all(query)
    }

    pub fn update_category(&self, category: CategoryEntity) -> Result<CategoryEntity, String> {
        self.repository.update(category)
    }

    pub fn delete_category(&self, id: Uuid) -> Result<(), String> {
        self.repository.delete(id)
    }

}