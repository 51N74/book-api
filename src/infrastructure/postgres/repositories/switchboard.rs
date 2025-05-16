use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use diesel::dsl::{delete, insert_into};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

use crate::domain::repositories::switchboard::SwitchboardRepository;
use crate::domain::value_objects::book_user_junction::BookUserJunction;
use crate::infrastructure::postgres::postgres_connection::PgPoolSquad;
use crate::infrastructure::postgres::schema::book_user_junction;



pub struct SwitchboardPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl SwitchboardPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl SwitchboardRepository for SwitchboardPostgres {
    async fn available(&self, junction_body: BookUserJunction) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        insert_into(book_user_junction::table)
            .values(junction_body)
            .execute(&mut conn)?;

        Ok(())
    }

    async fn reserved(&self, junction_body: BookUserJunction) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        delete(book_user_junction::table)
            .filter(book_user_junction::user_id.eq(junction_body.user_id))
            .filter(book_user_junction::book_id.eq(junction_body.book_id))
            .execute(&mut conn)?;

        Ok(())
    }

    // An example of a method that could be used for testing purposes
    fn for_transaction_test_1(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        junction_body: BookUserJunction,
    ) -> Result<()> {
        insert_into(book_user_junction::table)
            .values(junction_body)
            .execute(conn)?;

        Ok(())
    }

    fn for_transaction_test_2(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        junction_body: BookUserJunction,
    ) -> Result<()> {
        delete(book_user_junction::table)
            .filter(book_user_junction::user_id.eq(junction_body.user_id))
            .filter(book_user_junction::book_id.eq(junction_body.book_id))
            .execute(conn)?;

        Ok(())
    }
}