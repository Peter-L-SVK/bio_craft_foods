use axum::http::{header::HeaderName, Method}; 
use tower_http::cors::{CorsLayer, AllowOrigin};
use sqlx::MySqlPool;
use std::net::SocketAddr;
use dotenv::dotenv;
use axum::Router;
use tracing_subscriber;

mod utils;
mod routes;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok(); // Load the .env file

    // Set up the MariaDB connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await.unwrap();

    // Run migrations
    sqlx::migrate!().run(&pool).await.unwrap();

    // Create routes
    let app = Router::new()
        .nest("/api", routes::create_routes(pool))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::exact(
                    "http://localhost:3001".parse().unwrap(), // Parse the origin as a HeaderValue
                ))
                .allow_methods(vec![
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                ])
                .allow_headers(vec![HeaderName::from_static("content-type")]), // Use HeaderName
        );

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
