pub mod eval_error;

#[cfg(test)]
mod tests;

use crate::ast::*;
use crate::object::*;
use crate::object::environment::*;
use crate::token::*;
use eval_error::EvalError;

use boolean_literal::BooleanLiteral;
// use call_expression::CallExpression;
// use function_literal::FunctionLiteral;
use identifier::Identifier;
use if_expression::IfExpression;
use infix_expression::InfixExpression;
use integer_literal::IntegerLiteral;
use prefix_expression::PrefixExpression;
use block_statement::BlockStatement;
use let_statement::LetStatement;
use return_statement::ReturnStatement;
use std::rc::Rc;

pub const TRUE_OBJECT: Object = Object::Boolean(true);
pub const FALSE_OBJECT: Object = Object::Boolean(false);

pub trait EvalObject {
  fn eval(&self, env: &WrappedEnv) -> Result<Object, EvalError>;
}

impl EvalObject for Program {
  fn eval(&self, env: &WrappedEnv) -> Result<Object, EvalError> {
    let mut result = Object::Null;

    for statement in &self.statements {
      result = eval(statement, &Rc::clone(env))?;

      if let Object::Return(boxed_result) = result {
        return Ok(*boxed_result)
      }
    }

    Ok(result)
  }
}

impl EvalObject for Statement {
  fn eval(&self, env: &WrappedEnv) -> Result<Object, EvalError> {
    match &self {
      Statement::LetStatement(let_statement) => let_statement.eval(env),
      Statement::ReturnStatement(return_statement) => return_statement.eval(env),
      Statement::Expression(expression) => expression.eval(env),
      Statement::BlockStatement(block_statement) => block_statement.eval(env),
    }
  }
}

impl EvalObject for Expression {
  fn eval(&self, env: &WrappedEnv) -> Result<Object, EvalError> {
    match &self {
      Expression::Identifier(identifier) => identifier.eval(env),
      Expression::BooleanLiteral(boolean_literal) => boolean_literal.eval(env),
      Expression::IntegerLiteral(integer_literal) => integer_literal.eval(env),
      Expression::PrefixExpression(prefix_expression) => prefix_expression.eval(env),
      Expression::InfixExpression(infix_expression) => infix_expression.eval(env),
      Expression::IfExpression(if_expression) => if_expression.eval(env),
      Expression::FunctionLiteral(_function_literal) => Err(EvalError::not_implemented("FunctionLiteral")),
      Expression::CallExpression(_call_expression) => Err(EvalError::not_implemented("CallExpression")),
    }
  }
}

impl EvalObject for IntegerLiteral {
  fn eval(&self, env: &WrappedEnv) -> Result<Object, EvalError> {
    Ok(Object::Integer(self.value.clone()))
  }
}

impl EvalObject for BooleanLiteral {
  fn eval(&self, env: &WrappedEnv) -> Result<Object, EvalError> {
    Ok(native_boolean_to_boolean_object(self.value))
  }
}

impl EvalObject for PrefixExpression {
  fn eval(&self, env: &WrappedEnv) -> Result<Object, EvalError> {
    match self.operator.as_str() {
      token_types::BANG => eval_bang_operator_expression(&self.right, env),
      token_types::MINUS => eval_minus_operator_expression(&self.right, env),
      x => Err(EvalError::not_implemented(&format!("PrefixExpression for operator: {}", x))),
    }
  }
}

impl EvalObject for BlockStatement {
  fn eval(&self, env: &WrappedEnv) -> Result<Object, EvalError> {
    let mut result = Object::Null;

    for statement in &self.statements {
      result = statement.eval(&Rc::clone(env))?;

      if let Object::Return(_) = result {
        return Ok(result)
      }
    }

    Ok(result)
  }
}

impl EvalObject for IfExpression {
  fn eval(&self, env: &WrappedEnv) -> Result<Object, EvalError> {
    let condition_is_met = self.condition.eval(&Rc::clone(env))?.get_is_truthy().clone();

    if condition_is_met {
      self.true_block.eval(&Rc::clone(env))
    }
    else {
      match &*self.false_block_or_none {
        Some(false_block) => false_block.eval(env),
        None => Ok(Object::Null),
      }
    }
  }
}

impl EvalObject for ReturnStatement {
  fn eval(&self, env: &WrappedEnv) -> Result<Object, EvalError> {
    let return_object = self.return_value.eval(env)?;
    Ok(Object::Return(Box::new(return_object)))
  }
}

fn eval_bang_operator_expression(right: &Box<Expression>, env: &WrappedEnv) -> Result<Object, EvalError> {
  let right_object = right.eval(env)?;

  Ok(native_boolean_to_boolean_object(!*right_object.get_boolean_value()?))
}

fn eval_minus_operator_expression(right: &Box<Expression>, env: &WrappedEnv) -> Result<Object, EvalError> {
  let right_object = right.eval(env)?;
  let numeric_value = right_object.get_numeric_value()?;

  Ok(Object::Integer(-numeric_value))
}

impl EvalObject for InfixExpression {
  fn eval(&self, env: &WrappedEnv) -> Result<Object, EvalError> {
    let left_object = self.left.eval(&Rc::clone(env))?;
    let right_object = self.right.eval(&Rc::clone(env))?;

    match left_object {
      Object::Integer(_) => eval_integer_infix_expression(&self.operator, left_object, right_object),
      Object::Boolean(_) => eval_boolean_infix_expression(&self.operator, left_object, right_object),
      x => return Err(EvalError::not_implemented(&format!("InfixExpression for object type: {:?}", x))),
    }
  }
}

impl EvalObject for LetStatement {
  fn eval(&self, env: &WrappedEnv) -> Result<Object, EvalError> {
    let object = self.value.eval(env)?;

    env.borrow_mut().set(&self.name.value, object)
  }
}

impl EvalObject for Identifier {
  fn eval(&self, env: &WrappedEnv) -> Result<Object, EvalError> {
    env.borrow().get(&self.value)
  }
}

fn eval_integer_infix_expression(operator: &str, left: Object, right: Object) -> Result<Object, EvalError> {
  let left_value = left.get_numeric_value()?;
  let right_value = right.get_numeric_value()?;

  match operator {
    token_types::PLUS => Ok(Object::Integer(left_value + right_value)),
    token_types::MINUS => Ok(Object::Integer(left_value - right_value)),
    token_types::ASTERISK => Ok(Object::Integer(left_value * right_value)),
    token_types::SLASH => Ok(Object::Integer(left_value / right_value)),
    token_types::LT => Ok(native_boolean_to_boolean_object(left_value < right_value)),
    token_types::GT => Ok(native_boolean_to_boolean_object(left_value > right_value)),
    token_types::EQ => Ok(native_boolean_to_boolean_object(left_value == right_value)),
    token_types::NOT_EQ => Ok(native_boolean_to_boolean_object(left_value != right_value)),
    _ => Err(EvalError(format!("Unknown operation: Integer {} Integer", operator))),
  }
}

fn eval_boolean_infix_expression(operator: &str, left: Object, right: Object) -> Result<Object, EvalError> {
  let left_value = left.get_boolean_value()?;
  let right_value = right.get_boolean_value()?;

  match operator {
    token_types::EQ => Ok(native_boolean_to_boolean_object(left_value == right_value)),
    token_types::NOT_EQ => Ok(native_boolean_to_boolean_object(left_value != right_value)),
    _ => Err(EvalError(format!("Unknown operation: Boolean {} Boolean", operator))),
  }
}

fn native_boolean_to_boolean_object(boolean: bool) -> Object {
  if boolean {
    TRUE_OBJECT
  }
  else {
    FALSE_OBJECT
  }
}

pub fn eval(node: &impl EvalObject, env: &WrappedEnv) -> Result<Object, EvalError> {
  node.eval(env)
}
