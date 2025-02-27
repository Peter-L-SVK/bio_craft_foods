use axum::{Json, extract::{State, Path}, http::HeaderMap};
use serde_json::{json, Value};
use sqlx::MySqlPool;
use crate::models::product::{Product, CreateProduct};
use axum::http::StatusCode;
use validator::Validate;
use tracing::{info, error};

pub async fn list_products(State(pool): State<MySqlPool>) -> Result<(HeaderMap, Json<Value>), StatusCode> {
    info!("Fetching products from the database");

    let products = match sqlx::query_as::<_, Product>("SELECT id, name, description, price, in_stock FROM products")
        .fetch_all(&pool)
        .await
    {
        Ok(products) => products,
        Err(e) => {
            error!("Failed to fetch products: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    info!("Successfully fetched {} products", products.len());

    let mut headers = HeaderMap::new();
    let content_range = if products.is_empty() {
        "products */0".to_string()
    } else {
        format!("products 0-{}/{}", products.len() - 1, products.len())
    };

    headers.insert("Content-Range", content_range.parse().unwrap());

    Ok((headers, Json(json!({
        "data": products,
        "total": products.len(),
    }))))
}


pub async fn get_product(Path(id): Path<u32>, State(pool): State<MySqlPool>) -> Json<Value> {
    let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();

    Json(json!({ "data": product }))
}

pub async fn create_product(State(pool): State<MySqlPool>, Json(product): Json<CreateProduct>) -> Result<Json<Value>, StatusCode> {
    if let Err(_) = product.validate() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let _ = sqlx::query("INSERT INTO products (name, price, in_stock) VALUES (?, ?, ?)")
        .bind(&product.name)
        .bind(product.price)
        .bind(product.in_stock)
        .execute(&pool)
        .await
        .unwrap();

    Ok(Json(json!({ "message": "Product created successfully" })))
}

pub async fn update_product(Path(id): Path<u32>, State(pool): State<MySqlPool>, Json(product): Json<Product>) -> Json<Value> {
    let _ = sqlx::query("UPDATE products SET name = ?, description = ?, price = ?, in_stock = ? WHERE id = ?")
        .bind(&product.name)
        .bind(&product.description)
        .bind(product.price)
        .bind(product.in_stock)
        .bind(id)
        .execute(&pool)
        .await
        .unwrap();

    Json(json!({ "message": "Product updated successfully" }))
}

pub async fn delete_product(Path(id): Path<u32>, State(pool): State<MySqlPool>) -> Result<Json<Value>, StatusCode> {
    let result = sqlx::query("DELETE FROM products WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(result) if result.rows_affected() > 0 => Ok(Json(json!({ "message": "Product deleted successfully" }))),
        Ok(_) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

