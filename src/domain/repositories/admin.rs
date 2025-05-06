use anyhow::Result;
use axum::async_trait;
use mockall::automock;

use crate::domain::entities::admin::{AdminEntity, RegisterAdminEntity};

#[async_trait]
#[automock]
pub trait AdminRepository {
    async fn register(
        &self,
        register_admin_entity: RegisterAdminEntity,
    ) -> Result<i32>;
    async fn find_by_username(&self, username: String) -> Result<AdminEntity>;
}
