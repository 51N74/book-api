use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{domain::value_objects::book_model::BookModel, infrastructure::postgres::schema::books};



#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]

#[diesel(table_name = books)]
pub struct BookEntity {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub author: String,
    pub status: String,
    pub admin_id: i32,
    pub category_id: i32,
    pub price: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl BookEntity {
    pub fn to_model(&self, user_count: i32) -> BookModel {
        BookModel {
            id: self.id,
            title: self.title.clone(),
            description: self.description.clone(),
            author: self.author.clone(),
            price: self.price,
            category_id: self.category_id,
            status: self.status.clone(),
            admin_id: self.admin_id,
            user_count,
            created_at: self.created_at.unwrap_or_default(),
            updated_at: self.updated_at.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = books)]
pub struct AddBookEntity {
    pub title: String,
    pub description: Option<String>,
    pub author: String,
    pub category_id: i32,
    pub price: i32,
    pub status: String,
    pub admin_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, AsChangeset)]
#[diesel(table_name = books)]
pub struct EditBookEntity {
    pub title: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub status: Option<String>,
    pub category_id: Option<i32>,
    pub price: Option<i32>,
    pub admin_id: i32,
    pub updated_at: NaiveDateTime,
}