use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};

use crate::{application::services::category_viewing::CategoryViewingService, domain::repositories::category_viewing::CategoryViewingRepository, infrastructure::postgres::{postgres_connection::PgPoolSquad, repositories::category_viewing::CategoryViewingPostgres}};




pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let category_viewing_repository = CategoryViewingPostgres::new(db_pool);
    let category_viewing_service = CategoryViewingService::new(Arc::new(category_viewing_repository));

    Router::new()
        .route("/:category_id", get(view_details))
        // .route("/board-checking", get(board_checking))
        .with_state(Arc::new(category_viewing_service))
}

pub async fn view_details<T>(
    State(category_viewing_service): State<Arc<CategoryViewingService<T>>>,
    Path(category_id): Path<i32>,
) -> impl IntoResponse
where
    T: CategoryViewingRepository + Send + Sync,
{
    match category_viewing_service.view_details(category_id).await {
        Ok(category_model) => (StatusCode::OK, Json(category_model)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

// pub async fn board_checking<T>(
//     State(quest_viewing_use_case): State<Arc<QuestViewingUseCase<T>>>,
//     filter: Query<BoardCheckingFilter>,
// ) -> impl IntoResponse
// where
//     T: QuestViewingRepository + Send + Sync,
// {
//     match quest_viewing_use_case.board_checking(&filter).await {
//         Ok(quest_models) => (StatusCode::OK, Json(quest_models)).into_response(),
//         Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
//     }
// }