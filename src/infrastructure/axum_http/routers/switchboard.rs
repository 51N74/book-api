use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, post},
    Extension, Router,
};

use crate::{application::services::switchboard::SwitchboardService, domain::repositories::{book_viewing::BookViewingRepository, switchboard::SwitchboardRepository, transaction_provider::TransactionProvider}, infrastructure::postgres::{postgres_connection::PgPoolSquad, repositories::{book_viewing::BookViewingPostgres, diesel_transaction::DieselTransaction, switchboard::SwitchboardPostgres}}};

use super::middleware::user_authorization;



pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let switchboard_repository = SwitchboardPostgres::new(Arc::clone(&db_pool));
    let book_viewing_repository = BookViewingPostgres::new(Arc::clone(&db_pool));
    let tx = DieselTransaction::new(Arc::clone(&db_pool));

    let switchboard_service = SwitchboardService::new(
        Arc::new(switchboard_repository),
        Arc::new(book_viewing_repository),
        Arc::new(tx),
    );

    Router::new()
        .route("/available/:book_id", post(available))
        .route("/reserved/:book_id", delete(reserved))
        .route("/transaction-test/:book_id", post(transaction_test))
        .route_layer(middleware::from_fn(user_authorization))
        .with_state(Arc::new(switchboard_service))
}
 
pub async fn available<T1, T2, T3>(
    State(switchboard_service): State<Arc<SwitchboardService<T1, T2, T3>>>,
    Extension(user_id): Extension<i32>,
    Path(book_id): Path<i32>,
) -> impl IntoResponse
where
    T1: SwitchboardRepository + Send + Sync + 'static,
    T2: BookViewingRepository + Send + Sync,
    T3: TransactionProvider + Send + Sync,
{
    match switchboard_service
        .available(book_id, user_id)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            format!(
                "User id: {}, has Borrowing book id: {}",
                user_id, book_id
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn reserved<T1, T2, T3>(
    State(switchboard_service): State<Arc<SwitchboardService<T1, T2, T3>>>,
    Extension(user_id): Extension<i32>,
    Path(book_id): Path<i32>,
) -> impl IntoResponse
where
    T1: SwitchboardRepository + Send + Sync + 'static,
    T2: BookViewingRepository + Send + Sync,
    T3: TransactionProvider + Send + Sync,
{
    match switchboard_service
        .reserved(book_id, user_id)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            format!(
                "User id: {}, has Reserved book id: {}",
                user_id, book_id
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn transaction_test<T1, T2, T3>(
    State(switchboard_service): State<Arc<SwitchboardService<T1, T2, T3>>>,
    Extension(user_id): Extension<i32>,
    Path(book_id): Path<i32>,
) -> impl IntoResponse
where
    T1: SwitchboardRepository + Send + Sync + 'static,
    T2: BookViewingRepository + Send + Sync,
    T3: TransactionProvider + Send + Sync,
{
    match switchboard_service
        .available_and_reserved_transaction(book_id, user_id)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            format!(
                "User id: {}, has Available and Reserved book id: {}",
                user_id, book_id
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}