use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use diesel::dsl::insert_into;
use diesel::prelude::*;

use crate::domain::entities::category::{AddCategoryEntity, EditCategoryEntity};
use crate::domain::repositories::category_ops::CategoryOpsRepository;
use crate::domain::value_objects::category_statuses::CategoryStatuses;
use crate::infrastructure::postgres::postgres_connection::PgPoolSquad;
use crate::infrastructure::postgres::schema::categories;


pub struct CategoryOpsPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CategoryOpsPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl CategoryOpsRepository for CategoryOpsPostgres {
    async fn add(&self, add_category_entity: AddCategoryEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(categories::table)
            .values(add_category_entity)
            .returning(categories::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }

    async fn edit(&self, category_id: i32, edit_category_entity: EditCategoryEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = diesel::update(categories::table)
            .filter(categories::id.eq(category_id))
            .set(edit_category_entity)
            .returning(categories::id)
            .get_result::<i32>(&mut conn)?;

        Ok(result)
    }


    async fn remove(&self, category_id: i32, admin_id: i32) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        diesel::update(categories::table)
            .filter(categories::id.eq(category_id))
            .filter(categories::deleted_at.is_null())
            .filter(categories::status.eq(CategoryStatuses::Open.to_string()))
            .set((
                categories::deleted_at.eq(diesel::dsl::now),
                categories::admin_id.eq(admin_id),
            ))
            .execute(&mut conn)?;

        Ok(())
    }
}
