use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use diesel::prelude::*;

use crate::{domain::{entities::category::CategoryEntity, repositories::category_viewing::CategoryViewingRepository, value_objects::category_viewing_filter::CategoryViewingFilter}, infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::{books_categories_junction, categories}}};


pub struct CategoryViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CategoryViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CategoryViewingRepository for CategoryViewingPostgres {
    async fn view_details(&self, category_id: i32) -> Result<CategoryEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = categories::table
        .filter(categories::id.eq(category_id))
    .filter(categories::deleted_at.is_null())
    .select(CategoryEntity::as_select())  // ❌ cuses the problem
    .first::<CategoryEntity>(&mut conn)?;

        Ok(result)
    }

    async fn category_viewing(&self, filter: &CategoryViewingFilter) -> Result<Vec<CategoryEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let mut query = categories::table
            .filter(categories::deleted_at.is_null())
            .into_boxed();

        if let Some(status) = &filter.status {
            query = query.filter(categories::status.eq(status.to_string()));
        };

        if let Some(name) = &filter.name {
            query = query.filter(categories::name.ilike(format!("%{}%", name)));
        }

        let results = query
            .select(CategoryEntity::as_select())
            .order_by(categories::created_at.desc())
            .load::<CategoryEntity>(&mut conn)?;

        Ok(results)
    }

    async fn book_counting_by_category_id(&self, category_id: i32) -> Result<i64> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = books_categories_junction::table
            .filter(books_categories_junction::book_id.eq(category_id))
            .count()
            .first::<i64>(&mut conn)?;

        Ok(result)
    }
}