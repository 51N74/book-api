use std::sync::Arc;

use axum::async_trait;
use anyhow::Result;
use uuid::Uuid;
use crate::{domain::{entities::book::BookEntity, repositories::book::BookRepository}, infrastructure::postgres::postgres_connection::PgPoolSquad};

pub struct BookPostgres{
    db_pool:Arc<PgPoolSquad>
}

impl BookPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool} 
    }
}

#[async_trait]
impl BookRepository for BookPostgres {
    async fn create(&self, book: BookEntity) -> Result<i32>{
        unimplemented!()
    }
    async fn find_by_id(&self, id: Uuid) -> Result<Option<BookEntity>, String>{
        unimplemented!()
    }
    async fn find_all(&self, query: Option<String>) -> Result<Vec<BookEntity>, String>{
        unimplemented!()
    }
    async fn update(&self, book: BookEntity) -> Result<BookEntity, String>{
        unimplemented!()
    }
    async fn delete(&self, id: Uuid) -> Result<(), String>{
        unimplemented!()
    }
}