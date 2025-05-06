use std::sync::Arc;

use anyhow::Result;
use chrono::{Duration, Utc};

use crate::{config::config_loader::{get_admin_secret_env, get_user_secret_env}, domain::repositories::{admin::AdminRepository, user::UserRepository}, infrastructure::{argon2_hashing, jwt_authentication::{self, authentication_model::LoginModel, jwt_model::{Claims, Passport, Roles}}}};


pub struct AuthenticationService<T1, T2>
where
    T1: UserRepository + Send + Sync,
    T2: AdminRepository + Send + Sync,
{
    user_repository: Arc<T1>,
    admin_repository: Arc<T2>,
}

impl<T1, T2> AuthenticationService<T1, T2>
where
    T1: UserRepository + Send + Sync,
    T2: AdminRepository + Send + Sync,
{
    pub fn new(user_repository: Arc<T1>, admin_repository: Arc<T2>) -> Self {
        Self {
            user_repository,
            admin_repository,
        }
    }

    pub async fn user_login(&self, login_model: LoginModel) -> Result<Passport> {
        let secret_env = get_user_secret_env()?;

        let user = self
            .user_repository
            .find_by_username(login_model.username.clone())
            .await?;

        let original_password = user.password;
        let login_password = login_model.password;

        if !argon2_hashing::verify(login_password, original_password)? {
            return Err(anyhow::anyhow!("Invalid password"));
        }

        let access_token_claims = Claims {
            sub: user.id.to_string(),
            role: Roles::User,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: user.id.to_string(),
            role: Roles::User,
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token =
            jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;

        let refresh_token =
            jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            refresh_token,
            access_token,
        })
    }

    pub async fn user_refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let secret_env = get_user_secret_env()?;

        let claims = jwt_authentication::verify_token(
            secret_env.refresh_secret.clone(),
            refresh_token.clone(),
        )?;

        let access_token_claims = Claims {
            sub: claims.sub.clone(),
            role: Roles::User,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: claims.sub,
            role: Roles::User,
            exp: claims.exp,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token =
            jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;

        let refresh_token =
            jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            refresh_token,
            access_token,
        })
    }

    pub async fn admin_login(&self, login_model: LoginModel) -> Result<Passport> {
        let admin = self
            .admin_repository
            .find_by_username(login_model.username.clone())
            .await?;

        let original_password = admin.password;
        let login_password = login_model.password;

        if !argon2_hashing::verify(login_password, original_password)? {
            return Err(anyhow::anyhow!("Invalid password"));
        }

        let access_token_claims = Claims {
            sub: admin.id.to_string(),
            role: Roles::Admin,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: admin.id.to_string(),
            role: Roles::Admin,
            exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let secret_env = get_admin_secret_env()?;

        let access_token =
            jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;

        let refresh_token =
            jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            refresh_token,
            access_token,
        })
    }

    pub async fn admin_refresh_token(&self, refresh_token: String) -> Result<Passport> {
        let secret_env = get_admin_secret_env()?;

        let claims = jwt_authentication::verify_token(
            secret_env.refresh_secret.clone(),
            refresh_token.clone(),
        )?;

        let access_token_claims = Claims {
            sub: claims.sub.clone(),
            role: Roles::Admin,
            exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let refresh_token_claims = Claims {
            sub: claims.sub,
            role: Roles::Admin,
            exp: claims.exp,
            iat: Utc::now().timestamp() as usize,
        };

        let access_token =
            jwt_authentication::generate_token(secret_env.secret, &access_token_claims)?;

        let refresh_token =
            jwt_authentication::generate_token(secret_env.refresh_secret, &refresh_token_claims)?;

        Ok(Passport {
            refresh_token,
            access_token,
        })
    }
}