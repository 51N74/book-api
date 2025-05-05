use diesel::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::infrastructure::postgres::schema::users;

#[derive(Debug,Clone, Serialize, Deserialize,Identifiable,Selectable, Queryable)]
#[diesel(table_name = users)]
pub struct UserEntity{
    pub id:Uuid,
    pub username:String,
    pub password:String,
}

#[derive(Debug,Clone, Serialize, Deserialize,Identifiable,Selectable, Queryable)]
#[diesel(table_name = users)]
pub struct RegisterUserEntity{
    pub id:Uuid,
    pub username:String,
    pub password:String,
}