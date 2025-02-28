use axum::http::HeaderMap;
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::Error as SqlxError;
use validator::{ValidationError, ValidationErrors};
use rust_decimal::Decimal;
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use thiserror::Error;
use axum::Json;

// Custom error type for the application
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),
    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),
    #[error("Resource not found")]
    NotFound,
    #[allow(dead_code)]
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Internal server error")]
    InternalServerError,
}

// Implement `IntoResponse` for `AppError` to convert it into an HTTP response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DatabaseError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::ValidationError(_) => (StatusCode::BAD_REQUEST, "Validation error"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = json!({
            "error": error_message,
        });

        (status, Json(body)).into_response()
    }
}

// Utility function to generate JSON responses
pub fn json_response<T: Serialize>(data: T) -> Json<Value> {
    Json(json!({ "data": data }))
}

// Utility function to generate `Content-Range` headers
pub fn content_range_header(resource: &str, total: usize) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let content_range = if total == 0 {
        format!("{} */0", resource)
    } else {
        format!("{} 0-{}/{}", resource, total - 1, total)
    };
    headers.insert("Content-Range", content_range.parse().unwrap());
    headers
}

// Validation function for Decimal range
pub fn validate_decimal_range(value: &Decimal) -> Result<(), ValidationError> {
    let min = Decimal::new(0, 0);
    if *value < min {
        return Err(ValidationError::new("Value must be greater than or equal to the minimum"));
    }
    Ok(())
}

// Validation function to check if a customer exists
pub async fn validate_customer_exists(pool: &sqlx::MySqlPool, customer_id: i32) -> Result<(), AppError> {
    let exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM customers WHERE id = ?)")
        .bind(customer_id)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if !exists {
        return Err(AppError::NotFound);
    }
    Ok(())
}

// Validation function to check if a product exists
pub async fn validate_product_exists(pool: &sqlx::MySqlPool, product_id: i32) -> Result<(), AppError> {
    let exists = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM products WHERE id = ?)")
        .bind(product_id)
        .fetch_one(pool)
        .await
        .map_err(AppError::DatabaseError)?;

    if !exists {
        return Err(AppError::NotFound);
    }
    Ok(())
}

pub fn validate_date(date: &chrono::NaiveDate) -> Result<(), ValidationErrors> {
    let min_date = chrono::NaiveDate::from_ymd_opt(2020, 1, 1);
    if *date < min_date.expect("REASON") {
        let mut errors = ValidationErrors::new();
        errors.add(
            "order_date",
            ValidationError::new("Date must be after 2020-01-01"),
        );
        return Err(errors);
    }
    Ok(())
}
