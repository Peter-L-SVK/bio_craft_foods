use crate::utils::validate_decimal_range;
use serde::{Deserialize, Serialize};
use validator::Validate;
use sqlx::FromRow;
use rust_decimal::Decimal;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub in_stock: bool,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CreateProduct {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(custom(function = "validate_decimal_range"))]
    pub price: Decimal,
    pub in_stock: bool,
}
