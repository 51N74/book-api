use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::{application::services::user::UserService, domain::{repositories::user::UserRepository, value_objects::user_model::RegisterUserModel}, infrastructure::postgres::{postgres_connection::PgPoolSquad, repositories::user::UserPostgres}};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let user_repository = UserPostgres::new(db_pool);
    let user_service = UserService::new(Arc::new(user_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(user_service))
}

pub async fn register<T>(
    State(user_service): State<Arc<UserService<T>>>,
    Json(register_user_model): Json<RegisterUserModel>,
) -> impl IntoResponse
where
    T: UserRepository + Send + Sync,
{
    match user_service
        .register(register_user_model)
        .await
    {
        Ok(user_id) => (
            StatusCode::CREATED,
            format!("Register adventurer id: {} successfully", user_id),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}