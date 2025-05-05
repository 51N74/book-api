
use std::sync::Arc;

use uuid::Uuid;
use anyhow::Result;

use crate::domain::{repositories::user::UserRepository, value_objects::user_model::UserModel};

pub struct UserService<T>
where T: UserRepository + Send + Sync
{
    pub user_repository:Arc<T>
}

impl<T>UserService<T>
where T: UserRepository + Sync + Send 
 {
    pub fn new(user_repository: Arc<T>) -> Self {
        Self { user_repository }
    }

    pub async fn create(&self,mut register_user_model:UserModel ) ->Result<i32> {
        unimplemented!()
    }   
   }