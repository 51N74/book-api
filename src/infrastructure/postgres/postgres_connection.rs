use diesel::{r2d2::ConnectionManager, PgConnection,r2d2::Pool};
use anyhow::Result;
pub type PgPoolSquad = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url:&str) -> Result<PgPoolSquad> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool.");
    Ok(pool)
}