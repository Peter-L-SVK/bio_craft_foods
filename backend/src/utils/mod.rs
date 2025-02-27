use validator::ValidationError;
use rust_decimal::Decimal;

pub fn validate_decimal_range(value: &Decimal) -> Result<(), ValidationError> {
    let min = Decimal::new(0, 0);
    if *value < min {
        return Err(ValidationError::new("Value must be greater than or equal to the minimum"));
    }
    Ok(())
}

