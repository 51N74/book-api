use std::sync::Arc;

use axum::async_trait;
use anyhow::Result;
use uuid::Uuid;

use crate::{domain::{entities::user::UserEntity, repositories::user::{self, UserRepository}}, infrastructure::postgres::postgres_connection::PgPoolSquad};

pub struct UserPostgres{
    db_pool:Arc<PgPoolSquad>
}

impl UserPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool} 
    }
}

#[async_trait]
impl UserRepository for UserPostgres {
    async fn create(&self, user: UserEntity) -> Result<i32>{
        unimplemented!()
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserEntity>, String>{
        unimplemented!()
    }
    async fn find_all(&self, query: Option<String>) -> Result<Vec<UserEntity>, String>{
        unimplemented!()
    }
    async fn update(&self, user: UserEntity) -> Result<UserEntity, String>{
        unimplemented!()
    }
    async fn delete(&self, id: Uuid) -> Result<(), String>{
        unimplemented!()
    }
}