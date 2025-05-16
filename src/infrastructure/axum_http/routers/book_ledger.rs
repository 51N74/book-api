use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::patch,
    Extension, Router,
};

use crate::{application::services::book_ledger::BookLedgerService, domain::{repositories::{book_ledger::BookLedgerRepository, book_viewing::BookViewingRepository}, value_objects::book_statuses::BookStatuses}, infrastructure::postgres::{postgres_connection::PgPoolSquad, repositories::{book_ledger::BookLedgerPostgres, book_viewing::BookViewingPostgres}}};

use super::middleware::admin_authorization;


pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let book_ledger_repository = BookLedgerPostgres::new(Arc::clone(&db_pool));
    let book_viewing_repository = BookViewingPostgres::new(Arc::clone(&db_pool));
    let book_ledger_services = BookLedgerService::new(
        Arc::new(book_ledger_repository),
        Arc::new(book_viewing_repository),
    );

    Router::new()
        .route("/in-borrowing/:book_id", patch(in_borrowing))
        .route("/to-reserved/:book_id", patch(to_reserved))
        .route_layer(middleware::from_fn(admin_authorization))
        .with_state(Arc::new(book_ledger_services))
}

pub async fn in_borrowing<T1, T2>(
    State(book_ledger_services): State<Arc<BookLedgerService<T1, T2>>>,
    Extension(admin_id): Extension<i32>,
    Path(book_id): Path<i32>,
) -> impl IntoResponse
where
    T1: BookLedgerRepository + Send + Sync,
    T2: BookViewingRepository + Send + Sync,
{
    match book_ledger_services
        .in_borrowing(book_id, admin_id)
        .await
    {
        Ok(book_id) => (
            StatusCode::OK,
            format!(
                "Book id: {} is now {:?}",
                book_id,
                BookStatuses::Reserved
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn to_reserved<T1, T2>(
    State(book_ledger_services): State<Arc<BookLedgerService<T1, T2>>>,
    Extension(admin_id): Extension<i32>,
    Path(book_id): Path<i32>,
) -> impl IntoResponse
where
    T1: BookLedgerRepository + Send + Sync,
    T2: BookViewingRepository + Send + Sync,
{
    match book_ledger_services
        .to_reserved(book_id, admin_id)
        .await
    {
        Ok(book_id) => (
            StatusCode::OK,
            format!(
                "Book id: {} is now {:?}",
                book_id,
                BookStatuses::Available
            ),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

// pub async fn to_failed<T1, T2>(
//     State(journey_ledger_use_case): State<Arc<JourneyLedgerUseCase<T1, T2>>>,
//     Extension(guild_commander_id): Extension<i32>,
//     Path(quest_id): Path<i32>,
// ) -> impl IntoResponse
// where
//     T1: JourneyLedgerRepository + Send + Sync,
//     T2: QuestViewingRepository + Send + Sync,
// {
//     match journey_ledger_use_case
//         .to_failed(quest_id, guild_commander_id)
//         .await
//     {
//         Ok(quest_id) => (
//             StatusCode::OK,
//             format!("Quest id: {} is now {:?}", quest_id, QuestStatuses::Failed),
//         )
//             .into_response(),
//         Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
//     }
// }