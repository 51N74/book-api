use serde::{Deserialize, Serialize};

use crate::domain::entities::user::RegisterUserEntity;



#[derive(Debug, Clone,Serialize,Deserialize)]
pub struct RegisterUserModel {
    pub username: String,
    pub password: String,
}

impl RegisterUserModel{
    pub fn to_entity(&self)-> RegisterUserEntity{
        RegisterUserEntity{
            
            username: self.username.clone(),
            password: self.password.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}