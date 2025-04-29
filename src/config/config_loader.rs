use anyhow::Result;

use super::{
    config_model::{UserSecret, DotEnvyConfig, AdminSecret},
    stage::Stage,
};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = super::config_model::Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is invalid")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is invalid")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is invalid")
            .parse()?,
    };

    let database = super::config_model::Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid"),
    };

    Ok(DotEnvyConfig { server, database })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(&stage_str).unwrap_or_default()
}

pub fn get_user_secret_env() -> Result<UserSecret> {
    dotenvy::dotenv().ok();

    Ok(UserSecret {
        secret: std::env::var("JWT_USER_SECRET")?,
        refresh_secret: std::env::var("JWT_USER_REFRESH_SECRET")?,
    })
}

pub fn get_admin_secret_env() -> Result<AdminSecret> {
    dotenvy::dotenv().ok();

    Ok(AdminSecret {
        secret: std::env::var("JWT_ADMIN_SECRET")?,
        refresh_secret: std::env::var("JWT_ADMIN_REFRESH_SECRET")?,
    })
}