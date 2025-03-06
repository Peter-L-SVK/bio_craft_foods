use axum::{Json, extract::{State, Path}, http::HeaderMap};
use serde_json::Value;
use sqlx::MySqlPool;
use crate::models::product::{Product, CreateProduct};
use crate::utils::{AppError, json_response, content_range_header};
use validator::Validate;
use tracing::{info, error};
use validator::{ValidationErrors, ValidationError};

/// List all products
pub async fn list_products(State(pool): State<MySqlPool>) -> Result<(HeaderMap, Json<Value>), AppError> {
    info!("Fetching products from the database");

    let products = sqlx::query_as::<_, Product>("SELECT * FROM products")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch products: {:?}", e);
            AppError::DatabaseError(e)
        })?;

    info!("Successfully fetched {} products", products.len());
    let headers = content_range_header("products", products.len());
    Ok((headers, json_response(products)))
}

/// Get a specific product by ID
pub async fn get_product(Path(id): Path<u32>, State(pool): State<MySqlPool>) -> Result<Json<Value>, AppError> {
    let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| AppError::NotFound)?;

    Ok(json_response(product))
}

/// Create a new product
pub async fn create_product(State(pool): State<MySqlPool>, Json(product): Json<CreateProduct>) -> Result<Json<Value>, AppError> {
    // Validate the input
    product.validate().map_err(AppError::ValidationError)?;

    // Insert the new product into the database
    let _ = sqlx::query("INSERT INTO products (name, description, price, in_stock) VALUES (?, ?, ?, ?)")
        .bind(&product.name)
        .bind(&product.description) // Handle Option<String> properly
        .bind(product.price)
        .bind(product.in_stock)
        .execute(&pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(json_response("Product created successfully"))
}

/// Update an existing product
pub async fn update_product(Path(id): Path<u32>, State(pool): State<MySqlPool>, Json(product): Json<CreateProduct>) -> Result<Json<Value>, AppError> {
    // Validate the input
    product.validate().map_err(AppError::ValidationError)?;

    let _ = sqlx::query("UPDATE products SET name = ?, description = ?,   price = ?, in_stock = ? WHERE id = ?")
        .bind(&product.name)
	.bind(&product.description)
        .bind(product.price)
        .bind(product.in_stock)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(AppError::DatabaseError)?;

    Ok(json_response("Product updated successfully"))
}

/// Delete a product by ID
pub async fn delete_product(Path(id): Path<i32>, State(pool): State<MySqlPool>) -> Result<Json<Value>, AppError> {
     let result = sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(AppError::DatabaseError)?;

    // Check if the product was actually deleted
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(json_response("Product deleted successfully"))
}

/// Delete multiple products by IDs
pub async fn delete_products(State(pool): State<MySqlPool>, Json(ids): Json<Vec<i32>>) -> Result<Json<Value>, AppError> {
    if ids.is_empty() {
        let mut errors = ValidationErrors::new();
        errors.add("ids", ValidationError::new("No IDs provided"));
        return Err(AppError::ValidationError(errors));
    }

    // Construct the query with placeholders for each ID
    let query = format!(
        "DELETE FROM products WHERE id IN ({})",
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
