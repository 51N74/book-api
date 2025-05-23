use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::Result;
use axum::{http::Method, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;
use crate::{config::config_model::DotEnvyConfig, infrastructure::{axum_http::{default_routers, routers}, postgres::postgres_connection::PgPoolSquad}};

pub async fn start(config: Arc<DotEnvyConfig>, db_pool: Arc<PgPoolSquad>)->Result<()>{
    let app = Router::new()
    .fallback(default_routers::not_found)
     .nest(
            "/book-ledger",
            routers::book_ledger::routes(Arc::clone(&db_pool)),
        )
    .nest(
        "/book-ops",
        routers::book_ops::routes(Arc::clone(&db_pool)),
    )
    .nest(
        "/categpry-ops",
        routers::category_ops::routes(Arc::clone(&db_pool)),
    )
    .nest(
            "/switchboard",
            routers::switchboard::routes(Arc::clone(&db_pool)),
        )
    .nest(
        "/admin",
        routers::admin::routes(Arc::clone(&db_pool)),
    )
    .nest(
        "/users",
        routers::user::routes(Arc::clone(&db_pool)),
    )
    .nest(
            "/category-viewing",
            routers::category_viewing::routes(Arc::clone(&db_pool)),
        )
    .nest(
            "/book-viewing",
            routers::book_viewing::routes(Arc::clone(&db_pool)),
        )
    .nest(
        "/authentication",
        routers::authentication::routes(Arc::clone(&db_pool)),
    )
    .route("/health-check", get(default_routers::health_check))
    .layer(TimeoutLayer::new(Duration::from_secs(
        config.server.timeout,
    )))
    .layer(RequestBodyLimitLayer::new(
        (config.server.body_limit * 1024 * 1024).try_into()?,
    ))
    .layer(
        CorsLayer::new()
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::PATCH,
                Method::DELETE,
            ])
            .allow_origin(Any),
    )
    .layer(TraceLayer::new_for_http());

let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));

let listener = TcpListener::bind(addr).await?;

info!("Server running on port {}", config.server.port);

axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await?;

Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Received Ctrl+C signal"),
        _ = terminate => info!("Received terminate signal"),
    }
}