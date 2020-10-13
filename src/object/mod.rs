use crate::eval::eval_error::*;
use crate::ast::block_statement::*;
use environment::*;

pub mod environment;

#[derive(Debug, Clone)]
pub enum Object {
  Integer(i64),
  String(String),
  Boolean(bool),
  Null,
  Return(Box<Object>),
  Function(Vec<String>, Box<BlockStatement>, WrappedEnv),
}

impl Object {
  pub fn inspect(&self) -> String {
    match self {
      Object::Integer(integer) => integer.to_string(),
      Object::Boolean(is_true) => if *is_true { "True".to_string() } else { "False".to_string() },
      Object::Return(object) => object.inspect(),
      Object::Function(_, _, _) => "Function".to_string(),
      Object::String(string) => string.clone(),
      Object::Null => "Null".to_string(),
    }
  }

  pub fn get_boolean_value(&self) -> Result<&bool, EvalError> {
    match self {
      Object::Integer(integer) => if integer == &0 { Ok(&false) } else { Ok(&true) },
      Object::Boolean(is_true) => Ok(is_true),
      Object::Return(object) => object.get_boolean_value(),
      _ => Err(EvalError(format!("Expected boolean, found: {:?}", self))),
    }
  }

  pub fn get_numeric_value(&self) -> Result<i64, EvalError> {
    match self {
      Object::Integer(integer) => Ok(integer.clone()),
      Object::Return(object) => object.get_numeric_value(),
      _ => Err(EvalError(format!("Expected integer, found: {:?}", self))),
    }
  }

  pub fn get_string_value(&self) -> Result<String, EvalError> {
    match self {
      Object::String(string) => Ok(string.clone()),
      Object::Integer(integer) => Ok(integer.to_string()),
      Object::Return(object) => object.get_string_value(),
      _ => Err(EvalError(format!("Expected integer, found: {:?}", self))),
    }
  }

  pub fn get_is_truthy(&self) -> &bool {
    match self {
      Object::Integer(integer) => if integer == &0 { &false } else { &true },
      Object::String(_) => &true,
      Object::Boolean(is_true) => &is_true,
      Object::Return(object) => object.get_is_truthy(),
      Object::Function(_, _, _) => &true,
      Object::Null => &false,
    }
  }
}
