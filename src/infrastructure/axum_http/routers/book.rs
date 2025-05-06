use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use axum::extract::State;

use crate::{application::services::book::BookService, config::stage::Stage, domain::{repositories::book::BookRepository, value_objects::book_model::RegisterBookModel}, infrastructure::postgres::{postgres_connection::PgPoolSquad, repositories::book::BookPostgres}};

pub fn route(db_pool:Arc<PgPoolSquad>)->Router{
    let book_repository:BookPostgres = BookPostgres::new(db_pool);
    let book_service= BookService::new(Arc::new(book_repository));
    
         Router::new()
        // .route("/books", post(create::<BookPostgres>))
        .with_state(Arc::new(book_service))
}


pub async fn create<T>(
    State(book_service): State<Arc<BookService<T>>>,
    Json(register_book_model): Json<RegisterBookModel>,
) -> impl IntoResponse
where
    T: BookRepository + Send + Sync,
{
    match book_service
        .create(register_book_model)
        .await
    {
        Ok(adventurer_id) => (
            StatusCode::CREATED,
            format!("Register adventurer id: {} successfully", adventurer_id),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}