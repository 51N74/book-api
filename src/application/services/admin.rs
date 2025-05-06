use std::sync::Arc;

use anyhow::Result;

use crate::{domain::{repositories::admin::AdminRepository, value_objects::admin_model::RegisterAdminModel}, infrastructure::argon2_hashing};


pub struct AdminService<T>
where
    T: AdminRepository + Send + Sync,
{
    admin_repository: Arc<T>,
}

impl<T> AdminService<T>
where
    T: AdminRepository + Send + Sync,
{
    pub fn new(admin_repository: Arc<T>) -> Self {
        Self {
            admin_repository,
        }
    }

    pub async fn register(
        &self,
        mut register_admin_model: RegisterAdminModel,
    ) -> Result<i32> {
        let hashed_password =
            argon2_hashing::hash(register_admin_model.password.clone())?;

            register_admin_model.password = hashed_password;

        let register_entity = register_admin_model.to_entity();

        let guild_commander_id = self
            .admin_repository
            .register(register_entity)
            .await?;

        Ok(guild_commander_id)
    }
}