use std::sync::Arc;

use anyhow::Result;

use crate::domain::{repositories::category_viewing::CategoryViewingRepository, value_objects::{category_model::CategoryModel, category_viewing_filter::CategoryViewingFilter}};



pub struct CategoryViewingService<T>
where
    T: CategoryViewingRepository + Send + Sync,
{
    category_viewing_repository: Arc<T>,
}

impl<T> CategoryViewingService<T>
where
T: CategoryViewingRepository + Send + Sync,
{
    pub fn new(category_viewing_repository: Arc<T>) -> Self {
        Self {
            category_viewing_repository,
        }
    }

    pub async fn view_details(&self, category_id: i32) -> Result<CategoryModel> {
        let result = self.category_viewing_repository.view_details(category_id).await?;

        let user_count = self
            .category_viewing_repository
            .book_counting_by_category_id(result.id)
            .await?;

        let category_model = result.to_model(user_count.try_into().unwrap());

        Ok(category_model)
    }

    pub async fn category_viewing(&self, filter: &CategoryViewingFilter) -> Result<Vec<CategoryModel>> {
        let results = self.category_viewing_repository.category_viewing(filter).await?;

        let mut category_model = Vec::new();

        for result in results.into_iter() {
            let user_count = self
                .category_viewing_repository
                .book_counting_by_category_id(result.id)
                .await
                .unwrap_or(0);

            category_model.push(result.to_model(user_count.try_into().unwrap()));
        }

        Ok(category_model)
    }
}