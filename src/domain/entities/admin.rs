use chrono::NaiveDateTime;
use diesel::prelude::*;


use crate::infrastructure::postgres::schema::admin;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = admin)]
pub struct AdminEntity {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = admin)]
pub struct RegisterAdminEntity {
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}