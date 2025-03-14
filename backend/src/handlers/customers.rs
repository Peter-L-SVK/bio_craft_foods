use axum::{Json, extract::{State, Path}, http::HeaderMap};
use serde_json::Value;
use sqlx::MySqlPool;
use crate::models::customer::{Customer, CreateCustomer};
use crate::utils::{AppError, json_response, content_range_header, validate_customer_exists};
use validator::Validate;
use tracing::{info, error};
use validator::{ValidationErrors, ValidationError};

//noinspection ALL
/// List all customers
pub async fn list_customers(State(pool): State<MySqlPool>) -> Result<(HeaderMap, Json<Value>), AppError> {
    let customers = sqlx::query_as::<_, Customer>("SELECT * FROM customers")
        .fetch_all(&pool)
        .await
         .map_err(|e| {
	    error!("Failed to fetch customers: {:?}", e);
	    AppError::DatabaseError(e)
	})?;

    info!("Successfully fetched {} customers", customers.len());
    let headers = content_range_header("customers", customers.len());
    Ok((headers, json_response(customers)))
}

//noinspection ALL
/// Get a specific customer by ID
pub async fn get_customer(Path(id): Path<i32>, State(pool): State<MySqlPool>) -> Result<Json<Value>, AppError> {
    let customer = sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| AppError::NotFound)?;

    Ok(json_response(customer))
}

/// Create a new customer
pub async fn create_customer(State(pool): State<MySqlPool>, Json(customer): Json<CreateCustomer>) -> Result<Json<Value>, AppError> {
    // Validate the input
    customer.validate().map_err(AppError::ValidationError)?;

    // Insert the new customer into the database
    let _ = sqlx::query("INSERT INTO customers (name, email, address) VALUES (?, ?, ?)")
        .bind(&customer.name)
        .bind(&customer.email)
        .bind(&customer.address)
        .execute(&pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(json_response("Customer created successfully"))
}

/// Update an existing customer
pub async fn update_customer(Path(id): Path<i32>, State(pool): State<MySqlPool>, Json(customer): Json<CreateCustomer>) -> Result<Json<Value>, AppError> {
    // Validate the input
    customer.validate().map_err(AppError::ValidationError)?;

    // Check if the customer exists
    validate_customer_exists(&pool, id).await?;

    // Update the customer in the database
    let _ = sqlx::query("UPDATE customers SET name = ?, email = ?, address = ? WHERE id = ?")
        .bind(&customer.name)
        .bind(&customer.email)
        .bind(&customer.address)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(json_response("Customer updated successfully"))
}

/// Delete a customer by ID
pub async fn delete_customer(Path(id): Path<i32>, State(pool): State<MySqlPool>) -> Result<Json<Value>, AppError> {
    // Check if the customer exists
    validate_customer_exists(&pool, id).await?;

    // Delete the customer from the database
    let _ = sqlx::query("DELETE FROM customers WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(json_response("Customer deleted successfully"))
}

/// Delete multiple customers by IDs
pub async fn delete_customers(State(pool): State<MySqlPool>, Json(ids): Json<Vec<i32>>) -> Result<Json<Value>, AppError> {
    if ids.is_empty() {
        let mut errors = ValidationErrors::new();
        errors.add("ids", ValidationError::new("No IDs provided"));
        return Err(AppError::ValidationError(errors));
    }

    // Construct the query with placeholders for each ID
    let query = format!(
        "DELETE FROM customers WHERE id IN ({})",
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
