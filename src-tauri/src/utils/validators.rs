// Data validation utilities
// Input sanitization, error checking, etc.

pub fn validate_recipe_name(name: &str) -> Result<(), String> {
  if name.is_empty() {
    return Err("Recipe name cannot be empty".to_string());
  }
  if name.len() > 255 {
    return Err("Recipe name too long".to_string());
  }
  Ok(())
}

pub fn validate_quantity(qty: i32) -> Result<(), String> {
  if qty < 0 {
    return Err("Quantity cannot be negative".to_string());
  }
  Ok(())
}
