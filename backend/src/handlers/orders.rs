use axum::{Json, extract::{State, Path}, http::HeaderMap};
use serde_json::{json, Value};
use sqlx::MySqlPool;
use crate::models::order::{Order, CreateOrder};
use axum::http::StatusCode;
use validator::Validate;
use tracing::error;

pub async fn list_orders(State(pool): State<MySqlPool>) -> Result<(HeaderMap, Json<Value>), StatusCode> {
    let orders = match sqlx::query_as::<_, Order>("SELECT id, customer_id, product_id, quantity, order_date FROM orders")
        .fetch_all(&pool)
        .await
    {
        Ok(orders) => orders,
        Err(e) => {
            error!("Failed to fetch orders: {:?}", e); // Use the error! macro
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let mut headers = HeaderMap::new();
    let content_range = if orders.is_empty() {
        "orders */0".to_string()
    } else {
        format!("orders 0-{}/{}", orders.len() - 1, orders.len())
    };

    headers.insert("Content-Range", content_range.parse().unwrap());

    Ok((headers, Json(json!({
        "data": orders,
        "total": orders.len(),
    }))))
}

pub async fn get_order(Path(id): Path<i32>, State(pool): State<MySqlPool>) -> Json<Value> {
    let order = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();

    Json(json!({ "data": order }))
}

pub async fn create_order(State(pool): State<MySqlPool>, Json(order): Json<CreateOrder>) -> Result<Json<Value>, StatusCode> {
    if let Err(_) = order.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let _ = sqlx::query("INSERT INTO orders (customer_id, product_id, quantity, order_date) VALUES (?, ?, ?, ?)")
        .bind(order.customer_id)
        .bind(order.product_id)
        .bind(order.quantity)
        .bind(order.order_date) // Bind as NaiveDate
        .execute(&pool)
        .await
        .unwrap();

    Ok(Json(json!({ "message": "Order created successfully" })))
}

pub async fn update_order(Path(id): Path<i32>, State(pool): State<MySqlPool>, Json(order): Json<CreateOrder>) -> Json<Value> {
    let _ = sqlx::query("UPDATE orders SET customer_id = ?, product_id = ?, quantity = ?, order_date = ? WHERE id = ?")
        .bind(order.customer_id)
        .bind(order.product_id)
        .bind(order.quantity)
        .bind(&order.order_date)
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();

    Json(json!({ "message": "Order updated successfully" }))
}

pub async fn delete_order(Path(id): Path<i32>, State(pool): State<MySqlPool>) -> Json<Value> {
    let _ = sqlx::query("DELETE FROM orders WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();

    Json(json!({ "message": "Order deleted successfully" }))
}
