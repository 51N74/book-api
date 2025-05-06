use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::entities::book::{AddBookEntity, EditBookEntity};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BookModel {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub author: String,
    pub status: String,
    pub price: i32,
    pub user_count: i32,
    pub category_id: i32,
    pub admin_id:i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddBookModel {
    pub title: String,
    pub description: Option<String>,
    pub author: String,
   
}

impl AddBookModel {
    pub fn to_entity(&self, admin_id: i32) -> AddBookEntity {
        AddBookEntity {
            title: self.title.clone(),
            description: self.description.clone(),
            author: self.author.clone(),
            admin_id,
            category_id:0,
            price:0,
            status: crate::domain::value_objects::book_statuses::BookStatuses::Open.to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditBookModel {
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub status: Option<String>,
    pub category_id: Option<i32>,
    pub price: Option<i32>,
}

impl EditBookModel {
    pub fn to_entity(&self, admin_id: i32) -> EditBookEntity {
        EditBookEntity {
            title: self.title.clone(),
            description: self.description.clone(),
            author: self.author.clone(),
            price: self.price.clone(),
            admin_id,
            updated_at: chrono::Utc::now().naive_utc(),
            status: self.status.clone(),
            category_id: self.category_id,

        }
    }
}