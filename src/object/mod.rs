use crate::eval::eval_error::*;

#[derive(Debug)]
pub enum Object {
  Integer(i64),
  Boolean(bool),
  Null,
}

impl Object {
  pub fn inspect(&self) -> String {
    match self {
      Object::Integer(integer) => integer.to_string(),
      Object::Boolean(is_true) => if *is_true { "True".to_string() } else { "False".to_string() },
      Object::Null => "Null".to_string(),
    }
  }

  pub fn get_boolean_value(&self) -> Result<&bool, EvalError> {
    match self {
      Object::Integer(integer) => if integer == &0 { Ok(&false) } else { Ok(&true) },
      Object::Boolean(is_true) => Ok(is_true),
      _ => Err(EvalError(format!("Expected boolean, found: {:?}", self))),
    }
  }

  pub fn get_numeric_value(&self) -> Result<i64, EvalError> {
    match self {
      Object::Integer(integer) => Ok(integer.clone()),
      _ => Err(EvalError(format!("Expected boolean, found: {:?}", self))),
    }
  }
}
