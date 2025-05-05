use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::user::UserEntity;



#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct UserModel{
    pub id:Uuid,
    pub username:String,
    pub password:String,
}

impl UserModel{
    pub fn to_entity(&self) -> UserEntity {
        UserEntity {
            id: self.id,
            username: self.username.clone(),
            password: self.password.clone(),
        }
    
}
}