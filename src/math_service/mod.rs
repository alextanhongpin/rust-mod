// Filename must be mod.rs
pub mod four;
pub struct Service {}

#[derive(Debug)]
pub enum MathError {
  ZeroDivisionError
}

// type annotations needed, so you can't just return
// Result<f64, MathError> directly from the function
pub type MathResult = Result<f64, MathError>;

impl Service {
  pub fn new() -> Self {
    Service{}
  }
  pub fn sum(&self, a: i64, b: i64) -> i64 { 
    a + b + four::gen_four() as i64
  }
  pub fn sum_static(a: i64, b: i64) -> i64 { 
    a + b
  }
  pub fn div_static(a: f64, b: f64) -> MathResult { 
    if b == 0.0 {
      Err(MathError::ZeroDivisionError)
    } else {
      Ok(a / b)
    }
  }
}