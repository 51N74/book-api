use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use diesel::prelude::*;

use crate::{domain::{entities::book::BookEntity, repositories::book_viewing::BookViewingRepository, value_objects::book_viewing_filter::BookViewingFilter}, infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::{book_user_junction, books}}};


pub struct BookViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl BookViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl BookViewingRepository for BookViewingPostgres {
    async fn view_details(&self, book_id: i32) -> Result<BookEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = books::table
        .filter(books::id.eq(book_id))
    .filter(books::deleted_at.is_null())
    .select(BookEntity::as_select())  // ❌ cuses the problem
    .first::<BookEntity>(&mut conn)?;

        Ok(result)
    }

    async fn book_viewing(&self, filter: &BookViewingFilter) -> Result<Vec<BookEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let mut query = books::table
            .filter(books::deleted_at.is_null())
            .into_boxed();

        if let Some(status) = &filter.status {
            query = query.filter(books::status.eq(status.to_string()));
        };

        if let Some(name) = &filter.title {
            query = query.filter(books::title.ilike(format!("%{}%", name)));
        }

        let results = query
            .select(BookEntity::as_select())
            .order_by(books::created_at.desc())
            .load::<BookEntity>(&mut conn)?;

        Ok(results)
    }

    async fn user_counting_by_book_id(&self, book_id: i32) -> Result<i64> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = book_user_junction::table
            .filter(book_user_junction::book_id.eq(book_id))
            .count()
            .first::<i64>(&mut conn)?;

        Ok(result)
    }
}