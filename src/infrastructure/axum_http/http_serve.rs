// pub use std::net::SocketAddr;
// use std::sync::Arc;

// use axum::{routing::get, Router};

// use crate::{config, infrastructure::postgres};


// #[tokio::main]
// pub async  fn start(dotenvy_env:Arc<start(config: Arc<DotEnvyConfig>, db_pool: Arc<PgPoolSquad>) -> Result<()> {
//     // initialize tracing
//     tracing_subscriber::fmt::init();

//     // build our application with a route
//     let app = Router::new()
        
//         .route("/", get(root));
        
        

//     // run our app with hyper, listening globally on port 3000
//     let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

// // basic handler that responds with a static string
// async fn root() -> &'static str {
//     "Hello, World!"
// }