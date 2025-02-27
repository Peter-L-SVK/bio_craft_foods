use axum::{Json, extract::{State, Path}, http::HeaderMap};
use serde_json::{json, Value};
use sqlx::MySqlPool;
use crate::models::customer::{Customer, CreateCustomer};
use axum::http::StatusCode;
use validator::Validate;

pub async fn list_customers(State(pool): State<MySqlPool>) -> (HeaderMap, Json<Value>) {
    let customers = sqlx::query_as::<_, Customer>("SELECT * FROM customers")
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut headers = HeaderMap::new();

    // Handle the case where there are no customers
    let content_range = if customers.is_empty() {
        "customers */0".to_string()
    } else {
        format!("customers 0-{}/{}", customers.len() - 1, customers.len())
    };

    headers.insert("Content-Range", content_range.parse().unwrap());

    (headers, Json(json!({
        "data": customers,
        "total": customers.len(),
    })))
}

pub async fn get_customer(Path(id): Path<i32>, State(pool): State<MySqlPool>) -> Json<Value> {
    let customer = sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();

    Json(json!({ "data": customer }))
}

pub async fn create_customer(State(pool): State<MySqlPool>, Json(customer): Json<CreateCustomer>) -> Result<Json<Value>, StatusCode> {
    if let Err(_) = customer.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let _ = sqlx::query("INSERT INTO customers (name, email, address) VALUES (?, ?, ?)")
        .bind(&customer.name)
        .bind(&customer.email)
        .bind(&customer.address)
        .execute(&pool)
        .await
        .unwrap();

    Ok(Json(json!({ "message": "Customer created successfully" })))
}

pub async fn update_customer(Path(id): Path<i32>, State(pool): State<MySqlPool>, Json(customer): Json<CreateCustomer>) -> Json<Value> {
    let _ = sqlx::query("UPDATE customers SET name = ?, email = ?, address = ? WHERE id = ?")
        .bind(&customer.name)
        .bind(&customer.email)
        .bind(&customer.address)
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();

    Json(json!({ "message": "Customer updated successfully" }))
}

pub async fn delete_customer(Path(id): Path<i32>, State(pool): State<MySqlPool>) -> Json<Value> {
    let _ = sqlx::query("DELETE FROM customers WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();

    Json(json!({ "message": "Customer deleted successfully" }))
}
