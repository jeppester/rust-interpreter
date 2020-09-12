use std::fmt;

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

pub trait Object: fmt::Debug {
  fn get_type(&self) -> ObjectType;
  fn inspect(&self) -> String;
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
