use std::sync::Arc;

use axum::async_trait;
use anyhow::Result;
use uuid::Uuid;

use crate::{domain::{entities::category::{self, CategoryEntity}, repositories::category::CategoryRepository}, infrastructure::postgres::postgres_connection::PgPoolSquad};


pub struct CategoryPostgres{
    db_pool:Arc<PgPoolSquad>
}

impl CategoryPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool} 
    }
}

#[async_trait]
impl CategoryRepository for CategoryPostgres {
    async fn create(&self, category: CategoryEntity) -> Result<i32>{
        unimplemented!()
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CategoryEntity>, String>{
        unimplemented!()
    }
    async fn find_all(&self, query: Option<String>) -> Result<Vec<CategoryEntity>, String>{
        unimplemented!()
    }
    async fn update(&self, category: CategoryEntity) -> Result<CategoryEntity, String>{
        unimplemented!()
    }
    async fn delete(&self, id: Uuid) -> Result<(), String>{
        unimplemented!()
    }
}