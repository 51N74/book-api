use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::book::BookEntity;

#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RegisterBookModel{
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub category_id: Uuid,
    pub price: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl RegisterBookModel {
    pub fn to_entity(&self) -> BookEntity {
        BookEntity {
            id: self.id,
            title: self.title.clone(),
            author: self.author.clone(),
            category_id: self.category_id,
            price: self.price,
            created_at: self.created_at,
            updated_at: self.updated_at,
}
    }
}