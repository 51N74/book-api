use diesel::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::infrastructure::postgres::schema::categories;

#[derive(Debug,Clone, Serialize, Deserialize,Identifiable,Selectable, Queryable)]
#[diesel(table_name = categories)]
pub struct CategoryEntity {
    pub id: Uuid,
    pub name: String,
    
}

#[derive(Debug,Clone, Serialize, Deserialize,Identifiable,Selectable, Queryable)]
#[diesel(table_name = categories)]
pub struct RegisterCategoryEntity {
    pub id: Uuid,
    pub name: String,
}

