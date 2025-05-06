use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;

use diesel::{dsl::insert_into, prelude::*};

use crate::{domain::{entities::admin::{AdminEntity, RegisterAdminEntity}, repositories::admin::AdminRepository}, infrastructure::postgres::{postgres_connection::PgPoolSquad, schema::admin}};


pub struct AdminPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl AdminPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl AdminRepository for AdminPostgres {
    async fn register(
        &self,
        register_guild_commander_entity: RegisterAdminEntity,
    ) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(admin::table)
            .values(register_guild_commander_entity)
            .returning(admin::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<AdminEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = admin::table
            .filter(admin::username.eq(username))
            .select(AdminEntity::as_select())
            .first::<AdminEntity>(&mut conn)?;

        Ok(result)
    }
}