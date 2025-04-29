use std::sync::Arc;

use book_api::{
    config::config_loader,
    // infrastructure::{axum_http::http_serve::start, postgres::postgres_connection},
    infrastructure::{postgres::postgres_connection},
};
use tracing::info;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(e) => {
            tracing::error!("Failed to load environment variables: {}", e);
            std::process::exit(1);
        }
    };

    info!("Env has been loaded");

    let postgres_pool = match postgres_connection::establish_connection(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("Failed to establish connection: {}", e);
            std::process::exit(1);
        }
    };

    info!("Postgres connection has been established");

    // start(Arc::new(dotenvy_env), Arc::new(postgres_pool))
    //     .await
    //     .expect("Failed to start the server");

    start()
  
        
}


#[tokio::main]
async fn start() {
    
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));
        // `POST /users` goes to `create_user`
        

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
