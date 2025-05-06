use std::sync::Arc;

use anyhow::Result;

use crate::{domain::{repositories::user::UserRepository, value_objects::user_model::RegisterUserModel}, infrastructure::argon2_hashing};


pub struct UserService<T>
where
    T: UserRepository + Send + Sync,
{
    user_repository: Arc<T>,
}

impl<T> UserService<T>
where
    T: UserRepository + Send + Sync,
{
    pub fn new(user_repository: Arc<T>) -> Self {
        Self {
            user_repository,
        }
    }

    pub async fn register(
        &self,
        mut register_user_model: RegisterUserModel,
    ) -> Result<i32> {
        let hashed_password = argon2_hashing::hash(register_user_model.password.clone())?;

        register_user_model.password = hashed_password;

        let register_entity = register_user_model.to_entity();

        let user_id = self
            .user_repository
            .register(register_entity)
            .await?;

        Ok(user_id)
    }
}