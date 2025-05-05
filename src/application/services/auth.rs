
use uuid::Uuid;

use crate::domain::{entities::user::UserEntity, repositories::user::UserRepository,};





pub struct UserService<T: UserRepository> {
    repository: T,
}

impl<T: UserRepository> UserService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn add_user(&self, user: UserEntity) -> Result<UserEntity, String> {
        self.repository.create(user)
    }

    pub fn get_user(&self, id: Uuid) -> Result<Option<UserEntity>, String> {
        self.repository.find_by_id(id)
    }

    pub fn search_user(&self, query: Option<String>) -> Result<Vec<UserEntity>, String> {
        self.repository.find_all(query)
    }

    pub fn update_user(&self, user: UserEntity) -> Result<UserEntity, String> {
        self.repository.update(user)
    }

    pub fn delete_user(&self, id: Uuid) -> Result<(), String> {
        self.repository.delete(id)
    }

}