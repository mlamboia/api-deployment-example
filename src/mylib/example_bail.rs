pub fn divide(a: f64, b: f64) -> Result<f64, String> {
  if b == 0.0 {
    return Err("Division by zero".to_string());
  }
  Ok(a / b)
}
