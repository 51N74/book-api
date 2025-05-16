use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{domain::entities::{book::BookEntity, user::UserEntity}, infrastructure::postgres::schema::book_user_junction};



pub const MAX_ADVENTURERS_PER_QUEST: i64 = 4;

#[derive(Debug, Clone, Serialize, Deserialize, Selectable, Insertable, Queryable, Associations)]
#[diesel(belongs_to(UserEntity, foreign_key = user_id))]
#[diesel(belongs_to(BookEntity, foreign_key = book_id))]
#[diesel(table_name = book_user_junction)]
pub struct BookUserJunction {
    pub book_id: i32,
    pub user_id: i32,
}