use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User{
    pub id:Uuid,
    pub username:String,
    pub password:String,
}