use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use diesel::dsl::insert_into;
use diesel::prelude::*;

use crate::domain::entities::book::{AddBookEntity, EditBookEntity};
use crate::domain::repositories::book_ops::BookOpsRepository;
use crate::domain::value_objects::book_statuses::BookStatuses;
use crate::infrastructure::postgres::postgres_connection::PgPoolSquad;
use crate::infrastructure::postgres::schema::books;

pub struct BookOpsPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl BookOpsPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl BookOpsRepository for BookOpsPostgres {
    async fn add(&self, add_book_entity: AddBookEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(books::table)
            .values(add_book_entity)
            .returning(books::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }

    async fn edit(&self, book_id: i32, edit_book_entity: EditBookEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = diesel::update(books::table)
            .filter(books::id.eq(book_id))
            .filter(books::deleted_at.is_null())
            .filter(books::status.eq(BookStatuses::Available.to_string()))
            .set(edit_book_entity)
            .returning(books::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }

    async fn remove(&self, book_id: i32, admin_id: i32) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        diesel::update(books::table)
            .filter(books::id.eq(book_id))
            .filter(books::deleted_at.is_null())
            .filter(books::status.eq(BookStatuses::Available.to_string()))
            .set((
                books::deleted_at.eq(diesel::dsl::now),
                books::admin_id.eq(admin_id),
            ))
            .execute(&mut conn)?;

        Ok(())
    }
}