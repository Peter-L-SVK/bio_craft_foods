use sqlx::MySqlPool;
#[allow(unused_imports)]
use axum::{Router, routing::{get, post, put, delete}};
use crate::handlers::{customers, orders, products};

pub fn create_routes(pool: MySqlPool) -> Router {
    Router::new()
        // Products routes
        .route("/products", get(products::list_products).post(products::create_product))
        .route("/products/bulk-delete", post(products::delete_products)) 
        .route("/products/:id", get(products::get_product).put(products::update_product).delete(products::delete_product))
        
        // Customers routes
        .route("/customers", get(customers::list_customers).post(customers::create_customer))
        .route("/customers/bulk-delete", post(customers::delete_customers)) 
        .route("/customers/:id", get(customers::get_customer).put(customers::update_customer).delete(customers::delete_customer))
        
        // Orders routes
        .route("/orders", get(orders::list_orders).post(orders::create_order))
        .route("/orders/bulk-delete", post(orders::delete_orders)) 
        .route("/orders/:id", get(orders::get_order).put(orders::update_order).delete(orders::delete_order))
        
        .with_state(pool)
}
