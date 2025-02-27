use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct CreateCustomer {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(email(message = "Email must be valid"))]
    pub email: String,
    pub address: String,
}
