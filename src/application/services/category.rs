
use std::sync::Arc;

use uuid::Uuid;
use anyhow::Result;
use crate::{domain::{entities::category::CategoryEntity, repositories::category::CategoryRepository, value_objects::category_model::CategoryModel}, infrastructure::postgres::schema::categories};



pub struct CategoryService<T>
where T: CategoryRepository + Send + Sync
{
    pub category_repository:Arc<T>
}

impl<T>CategoryService<T>
where T: CategoryRepository + Sync + Send 
 {
    pub fn new(category_repository: Arc<T>) -> Self {
        Self { category_repository }
    }

    pub async fn create(&self,mut register_category_model:CategoryModel ) ->Result<i32> {
        unimplemented!()
    }   
   }