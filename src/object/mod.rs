use std::fmt;
use crate::eval::eval_error::*;

#[derive(Debug)]
pub enum ObjectType {
  Integer,
  Boolean,
  Null,
}

impl ObjectType {
  pub fn to_string(&self) -> &str {
    match self {
      ObjectType::Integer => "Integer",
      ObjectType::Boolean => "Boolean",
      ObjectType::Null => "Null",
    }
  }
}

const TRUE: bool = true;
const FALSE: bool = false;

pub trait Object: fmt::Debug {
  fn get_type(&self) -> ObjectType;
  fn inspect(&self) -> String;

  fn get_boolean_value(&self) -> Result<&bool, EvalError> {
    Err(EvalError(format!("Expected boolean, found: {:?}", self.get_type())))
  }

  fn get_numeric_value(&self) -> Result<&i64, EvalError> {
    Err(EvalError(format!("Expected numeric, found: {:?}", self.get_type())))
  }
}

// Integer
#[derive(Debug)]
pub struct Integer {
  pub value: i64,
}

impl Object for Integer {
  fn get_type(&self) -> ObjectType {
    ObjectType::Integer
  }

  fn inspect(&self) -> String {
    self.value.to_string()
  }

  fn get_boolean_value(&self) -> Result<&bool, EvalError> {
    match self.value {
      0 => Ok(&FALSE),
      _ => Ok(&TRUE),
    }
  }

  fn get_numeric_value(&self) -> Result<&i64, EvalError> {
    Ok(&self.value)
  }
}

// Boolean
#[derive(Debug)]
pub struct Boolean {
  pub value: bool,
}

impl Object for Boolean {
  fn get_type(&self) -> ObjectType {
    ObjectType::Boolean
  }

  fn inspect(&self) -> String {
    if self.value { "true".to_string() } else { "false".to_string() }
  }

  fn get_boolean_value(&self) -> Result<&bool, EvalError> {
    Ok(&self.value)
  }
}

// Null
#[derive(Debug)]
pub struct Null;

impl Object for Null {
  fn get_type(&self) -> ObjectType {
    ObjectType::Null
  }

  fn inspect(&self) -> String {
    "null".to_string()
  }
}
