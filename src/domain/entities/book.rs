use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub category_id: Uuid,
    pub price: f64,
}