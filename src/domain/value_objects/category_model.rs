use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::category::CategoryEntity;

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct CategoryModel{
    pub id: Uuid,
    pub name: String,
}

impl CategoryModel {
    pub fn to_entity(&self) -> CategoryEntity {
        CategoryEntity {
            id: self.id,
            name: self.name.clone(),
        }
    
}
}