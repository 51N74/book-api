use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, patch, post},
    Extension, Json, Router,
};



use crate::{application::services::category_ops::CategoryOpsService, domain::{repositories::{category_ops::CategoryOpsRepository, category_viewing::CategoryViewingRepository}, value_objects::category_model::{AddCategoryModel, EditCategoryModel}}, infrastructure::postgres::{postgres_connection::PgPoolSquad, repositories::{category_ops::CategoryOpsPostgres, category_viewing::CategoryViewingPostgres}}};

use super::middleware::admin_authorization;


pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let category_ops_repository = CategoryOpsPostgres::new(Arc::clone(&db_pool));
    let category_viewing_repository = CategoryViewingPostgres::new(Arc::clone(&db_pool));

    let category_ops_service = CategoryOpsService::new(
        Arc::new(category_ops_repository),
        Arc::new(category_viewing_repository),
    );

    Router::new()
        .route("/", post(add))
        .route("/:category_id", patch(edit))
        .route("/:category_id", delete(remove))
        .route_layer(middleware::from_fn(admin_authorization))
        .with_state(Arc::new(category_ops_service))
}

pub async fn add<T1, T2>(
    State(category_ops_service): State<Arc<CategoryOpsService<T1, T2>>>,
    Extension(admin_id): Extension<i32>,
    Json(add_category_model): Json<AddCategoryModel>,
) -> impl IntoResponse
where
    T1: CategoryOpsRepository + Send + Sync,
    T2: CategoryViewingRepository + Send + Sync,
{
    match category_ops_service
        .add(admin_id, add_category_model)
        .await
    {
        Ok(category_id_result) => {
            let response = format!("Add category success with id: {}", category_id_result);
            (StatusCode::CREATED, response).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn edit<T1, T2>(
    State(category_ops_service): State<Arc<CategoryOpsService<T1, T2>>>,
    Extension(admin_id): Extension<i32>,
    Path(category_id): Path<i32>,
    Json(edit_category_model): Json<EditCategoryModel>,
) -> impl IntoResponse
where
T1: CategoryOpsRepository + Send + Sync,
T2: CategoryViewingRepository + Send + Sync,
{
    match category_ops_service
        .edit(category_id, admin_id, edit_category_model)
        .await
    {
        Ok(category_id_result) => {
            let response = format!("Edit Category success with id: {}", category_id_result);
            (StatusCode::OK, response).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn remove<T1, T2>(
    State(category_ops_service): State<Arc<CategoryOpsService<T1, T2>>>,
    Extension(admin_id): Extension<i32>,
    Path(category_id): Path<i32>,
) -> impl IntoResponse
where
T1: CategoryOpsRepository + Send + Sync,
T2: CategoryViewingRepository + Send + Sync,
{
    match category_ops_service
        .remove(category_id, admin_id)
        .await
    {
        Ok(_) => {
            let response = format!("Remove Category success with id: {}", category_id);
            (StatusCode::OK, response).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}