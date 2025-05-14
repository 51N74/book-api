use anyhow::Result;
use axum::async_trait;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use mockall::automock;

use crate::domain::value_objects::book_user_junction::BookUserJunction;



#[async_trait]
#[automock]
pub trait SwitchboardRepository {
    async fn available(&self, junction_body: BookUserJunction) -> Result<()>;
    async fn reserved(&self, junction_body: BookUserJunction) -> Result<()>;
    // An example of a method that could be used for testing purposes
    fn for_transaction_test_1(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        junction_body: BookUserJunction,
    ) -> Result<()>;
    fn for_transaction_test_2(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        junction_body: BookUserJunction,
    ) -> Result<()>;
}