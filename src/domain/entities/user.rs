use chrono::NaiveDateTime;
use diesel::prelude::*;


use crate::infrastructure::postgres::schema::users;

#[derive(Debug, Clone, Identifiable, Selectable, Queryable)]
#[diesel(table_name = users)]
pub struct UserEntity{
    pub id:i32,
    pub username:String,
    pub password:String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Queryable)]
#[diesel(table_name = users)]
pub struct RegisterUserEntity{
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}