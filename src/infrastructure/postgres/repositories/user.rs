use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use diesel::prelude::*;
use diesel::{insert_into, RunQueryDsl};

use crate::domain::entities::user::{RegisterUserEntity, UserEntity};
use crate::domain::repositories::user::UserRepository;
use crate::infrastructure::postgres::postgres_connection::PgPoolSquad;
use crate::infrastructure::postgres::schema::users;

pub struct UserPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl UserPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl UserRepository for UserPostgres {
    async fn register(&self, register_user_entity: RegisterUserEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(users::table)
            .values(&register_user_entity)
            .returning(users::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<UserEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = users::table
            .filter(users::username.eq(username))
            .select(UserEntity::as_select())
            .first::<UserEntity>(&mut conn)?;

        Ok(result)
    }
}