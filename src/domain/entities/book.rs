use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::infrastructure::postgres::schema::books;

#[derive(Debug,Clone, Serialize, Deserialize,Identifiable,Selectable, Queryable)]
#[diesel(table_name = books)]
pub struct BookEntity {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub category_id: Uuid,
    pub price: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    
}

#[derive(Debug,Clone, Serialize, Deserialize,Identifiable,Selectable, Queryable)]
#[diesel(table_name = books)]
pub struct RegisterBookEntity {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub category_id: Uuid,
    pub price: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    
}

#[derive(Debug,Clone, Serialize, Deserialize,Selectable, Queryable)]
#[diesel(table_name = books)]
pub struct EditBookEntity {
    pub title: Option<String>,
    pub author: Option<String>,
    pub category_id: Uuid,
    pub price: Option<f64>,
    pub updated_at: NaiveDateTime,
}