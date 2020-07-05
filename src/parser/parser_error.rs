use std::fmt;

pub struct ParserError(pub String);

impl From<std::num::ParseIntError> for ParserError {
  fn from(_error: std::num::ParseIntError) -> Self {
    ParserError("ParseIntError".to_string())
  }
}

impl fmt::Display for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl fmt::Debug for ParserError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "ParserError: {}", self)
  }
}
