use axum::async_trait;
use mockall::automock;


use crate::domain::entities::user::{RegisterUserEntity, UserEntity};
use anyhow::Result;

#[async_trait]
#[automock]
pub trait UserRepository {
    async fn register(&self, register_adventurer_entity: RegisterUserEntity) -> Result<i32>;
    async fn find_by_username(&self, username: String) -> Result<UserEntity>;
}
