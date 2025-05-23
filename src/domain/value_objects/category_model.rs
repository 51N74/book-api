use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::entities::category::{AddCategoryEntity, EditCategoryEntity};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CategoryModel {
    pub id: i32,
    pub name: String,
    pub status: String,
    pub book_id: Option<i32>,
    pub admin_id:i32,
    pub user_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCategoryModel {
    pub name: String,
    pub status: String,

}

impl AddCategoryModel {
    pub fn to_entity(&self,admin_id: i32) -> AddCategoryEntity {
        AddCategoryEntity {
            name: self.name.clone(),
            admin_id,
            status: crate::domain::value_objects::category_statuses::CategoryStatuses::Open.to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditCategoryModel {
    pub name: Option<String>,
}

impl EditCategoryModel {
    pub fn to_entity(&self,admin_id: i32) -> EditCategoryEntity {
        EditCategoryEntity {
            name: self.name.clone(),
            admin_id,
            status: None,
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}