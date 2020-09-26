use std::collections::HashMap;
use crate::eval::eval_error::*;
use crate::object::Object;
use std::rc::Rc;
use std::cell::RefCell;

pub type WrappedEnv = Rc<RefCell<Environment>>;

#[derive(Debug)]
pub struct Environment {
  store: HashMap<String, Object>,
}

impl Environment {
  pub fn new() -> Self {
    Self {
      store: HashMap::new(),
    }
  }

  pub fn get(&self, key: &str) -> Result<Object, EvalError> {
    match self.store.get(key) {
      // It would be nice to not use clone here, especially for functions, but it would likely
      // require objects to be owned by a different entity, or maybe Rc can be used?
      Some(value) => Ok(value.clone()),
      None => Err(EvalError(format!("Unknown identifier: {}", key)))
    }
  }

  pub fn set(&mut self, key: &str, value: Object) -> Result<Object, EvalError> {
    let previous_or_none = self.store.insert(key.to_string(), value);

    if let Some(_) = previous_or_none {
      return Err(EvalError(format!("Identifier has already been declared: {}", key)))
    }

    Ok(Object::Null)
  }
}
