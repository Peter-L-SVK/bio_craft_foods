use axum::{Json, extract::{State, Path}, http::HeaderMap};
use serde_json::Value;
use sqlx::MySqlPool;
use crate::models::order::{Order, CreateOrder};
use validator::{Validate, ValidationError, ValidationErrors};
use crate::utils::{AppError, json_response, content_range_header, validate_customer_exists, validate_product_exists};
use tracing::{info, error};

/// List all orders
pub async fn list_orders(State(pool): State<MySqlPool>) -> Result<(HeaderMap, Json<Value>), AppError> {
    let orders = sqlx::query_as::<_, Order>("SELECT * FROM orders")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
	    error!("Failed to fetch orders: {:?}", e);
	    AppError::DatabaseError(e)
	})?;
    info!("Successfully fetched {} orders", orders.len());
    let headers = content_range_header("orders", orders.len());
    Ok((headers, json_response(orders)))
}

/// Get a specific order by ID
pub async fn get_order(Path(id): Path<i32>, State(pool): State<MySqlPool>) -> Result<Json<Value>, AppError> {
    let order = sqlx::query_as::<_, Order>("SELECT * FROM orders WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| AppError::NotFound)?;

    Ok(json_response(order))
}

/// Create a new order
pub async fn create_order(State(pool): State<MySqlPool>, Json(order): Json<CreateOrder>) -> Result<Json<Value>, AppError> {
    // Validate the input
    order.validate().map_err(AppError::ValidationError)?;

    // Validate the order date
    crate::utils::validate_date(&order.order_date).map_err(|e| {
        let mut errors = ValidationErrors::new();
        errors.add("order_date", e); // Convert ValidationError to ValidationErrors
        AppError::ValidationError(errors)
    })?;

    // Check if the customer exists
    validate_customer_exists(&pool, order.customer_id).await?;

    // Check if the product exists
    validate_product_exists(&pool, order.product_id).await?;

    // Insert the new order into the database
    let _ = sqlx::query("INSERT INTO orders (customer_id, product_id, quantity, order_date) VALUES (?, ?, ?, ?)")
        .bind(order.customer_id)
        .bind(order.product_id)
        .bind(order.quantity)
        .bind(order.order_date)
        .execute(&pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(json_response("Order created successfully"))
}

/// Update an existing order
pub async fn update_order(Path(id): Path<i32>, State(pool): State<MySqlPool>, Json(order): Json<CreateOrder>) -> Result<Json<Value>, AppError> {
    // Validate the input
    order.validate().map_err(AppError::ValidationError)?;

    // Validate the order date
    crate::utils::validate_date(&order.order_date).map_err(|e| {
        let mut errors = ValidationErrors::new();
        errors.add("order_date", e); // Convert ValidationError to ValidationErrors
        AppError::ValidationError(errors)
    })?;

    // Check if the customer exists
    validate_customer_exists(&pool, order.customer_id).await?;

    // Check if the product exists
    validate_product_exists(&pool, order.product_id).await?;

    // Update the order in the database
    let _ = sqlx::query("UPDATE orders SET customer_id = ?, product_id = ?, quantity = ?, order_date = ? WHERE id = ?")
        .bind(order.customer_id)
        .bind(order.product_id)
        .bind(order.quantity)
        .bind(order.order_date)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(json_response("Order updated successfully"))
}

/// Delete an order by ID
pub async fn delete_order(Path(id): Path<i32>, State(pool): State<MySqlPool>) -> Result<Json<Value>, AppError> {
    // Delete the order from the database
    let result = sqlx::query("DELETE FROM orders WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(AppError::DatabaseError)?;

    // Check if the order was actually deleted
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(json_response("Order deleted successfully"))
}

/// Delete multiple orders by IDs
pub async fn delete_orders(State(pool): State<MySqlPool>, Json(ids): Json<Vec<i32>>) -> Result<Json<Value>, AppError> {
    if ids.is_empty() {
        let mut errors = ValidationErrors::new();
        errors.add("ids", ValidationError::new("No IDs provided"));
        return Err(AppError::ValidationError(errors));
    }

    // Construct the query with placeholders for each ID
    let query = format!(
        "DELETE FROM orders WHERE id IN ({})",
        ids.iter().map(|_| "?").collect::<Vec<_>>().join(",")
    );

    // Execute the query
    let mut query = sqlx::query(&query);
    for id in ids.iter() {
        query = query.bind(id);
    }

    let result = query
        .execute(&pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    // Return the deleted IDs in the `data` field
    Ok(json_response(ids))
}
