use axum::http::{header::HeaderName, Method};
use tower_http::cors::{CorsLayer, AllowOrigin};
use sqlx::MySqlPool;
use std::net::SocketAddr;
use dotenv::dotenv;
use axum::Router;
use tracing_subscriber;
use crate::utils::AppError;

mod utils;
mod routes;
mod handlers;
mod models;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    // Load environment variables from .env file
    dotenv().ok();

    // Set up the MariaDB/MySQL connection pool
    let database_url = std::env::var("DATABASE_URL").map_err(|_| AppError::InternalServerError)?;
    let pool = MySqlPool::connect(&database_url)
        .await
        .map_err(AppError::DatabaseError)?;

    // Run database migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(|e| {
            println!("Failed to run migrations: {:?}", e);
            AppError::DatabaseError(e.into()) // Convert MigrateError to sqlx::Error
        })?;

    // Create the Axum router with all routes
    let app = Router::new()
        .nest("/api", routes::create_routes(pool))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::exact(
                    "http://localhost:3001".parse().map_err(|_| AppError::InternalServerError)?,
                ))
                .allow_methods(vec![
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                ])
                .allow_headers(vec![HeaderName::from_static("content-type")]),
        );

    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|_| AppError::InternalServerError)?;

    Ok(())
}
