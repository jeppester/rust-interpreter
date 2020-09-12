use std::fmt;

pub struct EvalError(pub String);

impl EvalError {
  pub fn not_implemented() -> Self {
    EvalError("Not implemented".to_string())
  }
}

impl From<std::num::ParseIntError> for EvalError {
  fn from(_error: std::num::ParseIntError) -> Self {
    EvalError("ParseIntError".to_string())
  }
}

impl fmt::Display for EvalError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl fmt::Debug for EvalError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "EvalError: {}", self)
  }
}
