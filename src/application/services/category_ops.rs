use std::sync::Arc;

use anyhow::Result;

use crate::domain::{repositories::{category_ops::CategoryOpsRepository, category_viewing::CategoryViewingRepository}, value_objects::category_model::{AddCategoryModel, EditCategoryModel}};


pub struct CategoryOpsService<T1, T2>
where
    T1: CategoryOpsRepository + Send + Sync,
    T2: CategoryViewingRepository + Send + Sync,
{
    category_ops_repository: Arc<T1>,
    category_viewing_repository: Arc<T2>,
}

impl<T1, T2> CategoryOpsService<T1, T2>
where
T1: CategoryOpsRepository + Send + Sync,
T2: CategoryViewingRepository + Send + Sync,
{
    pub fn new(category_ops_repository: Arc<T1>, category_viewing_repository: Arc<T2>) -> Self {
        Self {
            category_ops_repository,
            category_viewing_repository,
        }
    }

    pub async fn add(
        &self,
        admin_id: i32,
        add_category_model: AddCategoryModel,
    ) -> Result<i32> {
        let insert_category_entity = add_category_model.to_entity(admin_id);

        let result = self.category_ops_repository.add(insert_category_entity).await?;

        Ok(result)
    }

    pub async fn edit(
        &self,
        category_id: i32,
        admin_id: i32,
        edit_category_model: EditCategoryModel,
    ) -> Result<i32> {
        // Check if adventurer exists
        let user_count = self
            .category_viewing_repository
            .book_counting_by_category_id(category_id)
            .await?;

        if user_count > 0 {
            return Err(anyhow::anyhow!(
                "Quest has been taken by adventurers for now!!!"
            ));
        }

        // Update category
        let edit_category_entity = edit_category_model.to_entity(admin_id);

        let result = self
            .category_ops_repository
            .edit(category_id, edit_category_entity)
            .await?;

        Ok(result)
    }

    pub async fn remove(&self, category_id: i32, admin_id: i32) -> Result<()> {
        // Check if adventurer exists
        let user_count = self
            .category_viewing_repository
            .book_counting_by_category_id(category_id)
            .await?;

        if user_count > 0 {
            return Err(anyhow::anyhow!(
                "Quest has been taken by adventurers for now!!!"
            ));
        }

        self.category_ops_repository
            .remove(category_id, admin_id)
            .await?;

        Ok(())
    }
}