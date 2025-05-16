use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use diesel::{ExpressionMethods, RunQueryDsl};

use crate::{domain::{repositories::book_ledger::BookLedgerRepository, value_objects::book_statuses::BookStatuses}, infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::books}};



pub struct BookLedgerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl BookLedgerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl BookLedgerRepository for BookLedgerPostgres {
    async fn in_borrowing(&self, book_id: i32, admin_id: i32) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = diesel::update(books::table)
            .filter(books::id.eq(book_id))
            .filter(books::deleted_at.is_null())
            .set((
                books::status.eq(BookStatuses::Available.to_string()),
                books::admin_id.eq(admin_id),
            ))
            .returning(books::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }

    async fn to_reserved(&self, book_id: i32, admin_id: i32) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = diesel::update(books::table)
            .filter(books::id.eq(book_id))
            .filter(books::deleted_at.is_null())
            .set((
                books::status.eq(BookStatuses::Reserved.to_string()),
                books::admin_id.eq(admin_id),
            ))
            .returning(books::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }

    // async fn to_failed(&self, quest_id: i32, guild_commander_id: i32) -> Result<i32> {
    //     let mut conn = Arc::clone(&self.db_pool).get()?;

    //     let result = diesel::update(quests::table)
    //         .filter(quests::id.eq(quest_id))
    //         .filter(quests::deleted_at.is_null())
    //         .set((
    //             quests::status.eq(QuestStatuses::Failed.to_string()),
    //             quests::guild_commander_id.eq(guild_commander_id),
    //         ))
    //         .returning(quests::id)
    //         .get_result::<i32>(&mut conn)?;

    //     Ok(result)
    // }
}