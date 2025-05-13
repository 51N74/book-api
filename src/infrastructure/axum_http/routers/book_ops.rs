use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, patch, post},
    Extension, Json, Router,
};

use crate::{application::services::book_ops::BookOpsService, domain::{repositories::{book_ops::BookOpsRepository, book_viewing::BookViewingRepository}, value_objects::book_model::{AddBookModel, EditBookModel}}, infrastructure::postgres::{postgres_connection::PgPoolSquad, repositories::{book_ops::BookOpsPostgres, book_viewing::BookViewingPostgres}}};

use super::middleware::admin_authorization;


pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let book_ops_repository = BookOpsPostgres::new(Arc::clone(&db_pool));
    let book_viewing_repository = BookViewingPostgres::new(Arc::clone(&db_pool));

    let book_ops_service = BookOpsService::new(
        Arc::new(book_ops_repository),
        Arc::new(book_viewing_repository),
    );

    Router::new()
        .route("/add", post(add))
        .route("/:book_id", patch(edit))
        .route("/:book_id", delete(remove))
        .route_layer(middleware::from_fn(admin_authorization))
        .with_state(Arc::new(book_ops_service))
}

pub async fn add<T1, T2>(
    State(book_ops_service): State<Arc<BookOpsService<T1, T2>>>,
    Extension(admin_id): Extension<i32>,
    Json(add_book_model): Json<AddBookModel>,
) -> impl IntoResponse
where
    T1: BookOpsRepository + Send + Sync,
    T2: BookViewingRepository + Send + Sync,
{
    match book_ops_service
        .add(admin_id, add_book_model)
        .await
    {
        Ok(book_id_result) => {
            let response = format!("Add quest success with id: {}", book_id_result);
            (StatusCode::CREATED, response).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn edit<T1, T2>(
    State(book_ops_service): State<Arc<BookOpsService<T1, T2>>>,
    Extension(admin_id): Extension<i32>,
    Path(book_id): Path<i32>,
    Json(edit_book_model): Json<EditBookModel>,
) -> impl IntoResponse
where
    T1: BookOpsRepository + Send + Sync,
    T2: BookViewingRepository + Send + Sync,
{
    match book_ops_service
        .edit(book_id, admin_id, edit_book_model)
        .await
    {
        Ok(book_id_result) => {
            let response = format!("Edit quest success with id: {}", book_id_result);
            (StatusCode::OK, response).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn remove<T1, T2>(
    State(book_ops_service): State<Arc<BookOpsService<T1, T2>>>,
    Extension(admin_id): Extension<i32>,
    Path(book_id): Path<i32>,
) -> impl IntoResponse
where
    T1: BookOpsRepository + Send + Sync,
    T2: BookViewingRepository + Send + Sync,
{
    match book_ops_service
        .remove(book_id, admin_id)
        .await
    {
        Ok(_) => {
            let response = format!("Remove quest success with id: {}", book_id);
            (StatusCode::OK, response).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}