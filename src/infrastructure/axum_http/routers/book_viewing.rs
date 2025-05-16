use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use crate::{application::services::book_viewing::BookViewingService, domain::{repositories::book_viewing::BookViewingRepository, value_objects::book_viewing_filter::BookViewingFilter}, infrastructure::postgres::{postgres_connection::PgPoolSquad, repositories::book_viewing::BookViewingPostgres}};


pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let book_viewing_repository = BookViewingPostgres::new(db_pool);
    let book_viewing_service = BookViewingService::new(Arc::new(book_viewing_repository));

    Router::new()
        .route("/:book_id", get(view_details))
        .route("/book-checking", get(book_checking))
        .with_state(Arc::new(book_viewing_service))
}

pub async fn view_details<T>(
    State(book_viewing_service): State<Arc<BookViewingService<T>>>,
    Path(book_id): Path<i32>,
) -> impl IntoResponse
where
    T: BookViewingRepository + Send + Sync,
{
    match book_viewing_service.view_details(book_id).await {
        Ok(book_model) => (StatusCode::OK, Json(book_model)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn book_checking<T>(
    State(book_viewing_service): State<Arc<BookViewingService<T>>>,
    filter: Query<BookViewingFilter>,
) -> impl IntoResponse
where
    T: BookViewingRepository + Send + Sync,
{
    match book_viewing_service.book_viewing(&filter).await {
        Ok(book_models) => (StatusCode::OK, Json(book_models)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}