use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{domain::value_objects::category_model::CategoryModel, infrastructure::postgres::schema::categories};



#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]

#[diesel(table_name = categories)]
pub struct CategoryEntity {
    pub id: i32,
    pub name: String,
    pub admin_id: i32,
    pub book_id: i32,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}

impl CategoryEntity {
    pub fn to_model(&self, user_count: i32) -> CategoryModel {
        CategoryModel {
            id: self.id,
            name: self.name.clone(),
            admin_id: self.admin_id,
            status: self.status.clone(),
            book_id: self.book_id,
            created_at: self.created_at.unwrap_or_default(),
            updated_at: self.updated_at.unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = categories)]
pub struct AddCategoryEntity {
    pub name: String,
    pub admin_id: i32,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, AsChangeset)]
#[diesel(table_name = categories)]
pub struct EditCategoryEntity {
    pub name: Option<String>,
    pub admin_id: i32,
    pub status: Option<String>,
    pub updated_at: NaiveDateTime,
}