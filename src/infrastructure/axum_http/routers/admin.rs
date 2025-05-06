use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};

use crate::{application::services::admin::AdminService, domain::{repositories::admin::AdminRepository, value_objects::admin_model::RegisterAdminModel}, infrastructure::postgres::{postgres_connection::PgPoolSquad, repositories::admin::AdminPostgres}};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let admin_repository = AdminPostgres::new(db_pool);
    let admin_service =
        AdminService::new(Arc::new(admin_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(admin_service))
}

pub async fn register<T>(
    State(admin_service): State<Arc<AdminService<T>>>,
    Json(register_admin_model): Json<RegisterAdminModel>,
) -> impl IntoResponse
where
    T: AdminRepository + Send + Sync,
{
    match admin_service
        .register(register_admin_model)
        .await
    {
        Ok(admin_id) => (
            StatusCode::CREATED,
            format!(
                "Register guild_commander id: {} successfully",
                admin_id
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}