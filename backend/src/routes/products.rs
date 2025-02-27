use axum::{Router, routing::get};
use sqlx::MySqlPool;
use crate::handlers::products;

pub fn create_routes(pool: MySqlPool) -> Router {
    Router::new()
        .route("/products", get({
            let pool = pool.clone();
            move || products::list_products(pool)
        }))
}
