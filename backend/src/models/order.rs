use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;
use chrono::NaiveDate;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Order {
    pub id: i32,
    pub customer_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub order_date: NaiveDate, 
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CreateOrder {
    #[validate(range(min = 1, message = "Customer ID must be a positive number"))]
    pub customer_id: i32,
    #[validate(range(min = 1, message = "Product ID must be a positive number"))]
    pub product_id: i32,
    #[validate(range(min = 1, message = "Quantity must be a positive number"))]
    pub quantity: i32,
    #[validate(custom(function = "validate_date"))]
    pub order_date: NaiveDate, // Added the type annotation here
}

fn validate_date(date: &NaiveDate) -> Result<(), validator::ValidationError> {
    if date < &NaiveDate::from_ymd_opt(2020, 1, 1).unwrap() {
        return Err(validator::ValidationError::new("Order date must be after 2020-01-01"));
    }
    Ok(())
}
