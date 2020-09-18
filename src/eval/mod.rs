pub mod eval_error;

#[cfg(test)]
mod tests;

use crate::ast::*;
use crate::object::*;
use crate::token::*;
use eval_error::EvalError;

use boolean_literal::BooleanLiteral;
// use call_expression::CallExpression;
// use function_literal::FunctionLiteral;
// use identifier::Identifier;
// use if_expression::IfExpression;
use infix_expression::InfixExpression;
use integer_literal::IntegerLiteral;
use prefix_expression::PrefixExpression;
// use block_statement::BlockStatement;
// use let_statement::LetStatement;
// use return_statement::ReturnStatement;

pub const TRUE_OBJECT: Boolean = Boolean { value: true };
pub const FALSE_OBJECT: Boolean = Boolean { value: false };
pub const NULL_OBJECT: Null = Null;

pub trait EvalObject {
  fn eval(&self) -> Result<Box<dyn Object>, EvalError>;
}

impl EvalObject for Program {
  fn eval(&self) -> Result<Box<dyn Object>, EvalError> {
    eval_statements(&self.statements)
  }
}

impl EvalObject for Statement {
  fn eval(&self) -> Result<Box<dyn Object>, EvalError> {
    match &self {
      Statement::LetStatement(_let_statement) => Err(EvalError::not_implemented("LetStatement")),
      Statement::ReturnStatement(_return_statement) => Err(EvalError::not_implemented("ReturnStatement")),
      Statement::Expression(expression) => expression.eval(),
      Statement::BlockStatement(_block_statement) => Err(EvalError::not_implemented("BlockStatement")),
    }
  }
}

impl EvalObject for Expression {
  fn eval(&self) -> Result<Box<dyn Object>, EvalError> {
    match &self {
      Expression::Identifier(_expression) => Err(EvalError::not_implemented("Expression")),
      Expression::BooleanLiteral(boolean_literal) => boolean_literal.eval(),
      Expression::IntegerLiteral(integer_literal) => integer_literal.eval(),
      Expression::PrefixExpression(prefix_expression) => prefix_expression.eval(),
      Expression::InfixExpression(infix_expression) => infix_expression.eval(),
      Expression::IfExpression(_if_expression) => Err(EvalError::not_implemented("IfExpression")),
      Expression::FunctionLiteral(_function_literal) => Err(EvalError::not_implemented("FunctionLiteral")),
      Expression::CallExpression(_call_expression) => Err(EvalError::not_implemented("CallExpression")),
    }
  }
}

impl EvalObject for IntegerLiteral {
  fn eval(&self) -> Result<Box<dyn Object>, EvalError> {
    Ok(Box::new(Integer { value: self.value.clone() }))
  }
}

impl EvalObject for BooleanLiteral {
  fn eval(&self) -> Result<Box<dyn Object>, EvalError> {
    Ok(native_boolean_to_boolean_object(self.value))
  }
}

impl EvalObject for PrefixExpression {
  fn eval(&self) -> Result<Box<dyn Object>, EvalError> {
    match self.operator.as_str() {
      token_types::BANG => eval_bang_operator_expression(&self.right),
      token_types::MINUS => eval_minus_operator_expression(&self.right),
      x => Err(EvalError::not_implemented(&format!("PrefixExpression for operator: {}", x))),
    }
  }
}

fn eval_bang_operator_expression(right: &Box<Expression>) -> Result<Box<dyn Object>, EvalError> {
  let right_object = right.eval()?;
  Ok(native_boolean_to_boolean_object(!*right_object.get_boolean_value()?))
}

fn eval_minus_operator_expression(right: &Box<Expression>) -> Result<Box<dyn Object>, EvalError> {
  let right_object = right.eval()?;
  let numeric_value = right_object.get_numeric_value()?;

  Ok(Box::new(Integer { value: -numeric_value }))
}

impl EvalObject for InfixExpression {
  fn eval(&self) -> Result<Box<dyn Object>, EvalError> {
    let left_object = self.left.eval()?;
    let right_object = self.right.eval()?;

    match left_object.get_type() {
      ObjectType::Integer => eval_integer_infix_expression(&self.operator, left_object, right_object),
      x => return Err(EvalError::not_implemented(&format!("InfixExpression for object type: {:?}", x))),
    }
  }
}

fn eval_integer_infix_expression(operator: &str, left: Box<dyn Object>, right: Box<dyn Object>) -> Result<Box<dyn Object>, EvalError> {
  let left_value = left.get_numeric_value()?;
  let right_value = right.get_numeric_value()?;

  match operator {
    token_types::PLUS => Ok(Box::new(Integer { value: left_value + right_value })),
    token_types::MINUS => Ok(Box::new(Integer { value: left_value - right_value })),
    token_types::ASTERISK => Ok(Box::new(Integer { value: left_value * right_value })),
    token_types::SLASH => Ok(Box::new(Integer { value: left_value / right_value })),
    token_types::LT => Ok(native_boolean_to_boolean_object(left_value < right_value)),
    token_types::GT => Ok(native_boolean_to_boolean_object(left_value > right_value)),
    token_types::EQ => Ok(native_boolean_to_boolean_object(left_value == right_value)),
    token_types::NOT_EQ => Ok(native_boolean_to_boolean_object(left_value != right_value)),
    x => Err(EvalError::not_implemented(&format!("InfixExpression for operator: {}", x))),
  }
}

fn native_boolean_to_boolean_object(boolean: bool) -> Box<dyn Object> {
  if boolean {
    Box::new(TRUE_OBJECT)
  }
  else {
    Box::new(FALSE_OBJECT)
  }
}

pub fn eval(node: &impl EvalObject) -> Result<Box<dyn Object>, EvalError> {
  node.eval()
}

pub fn eval_statements(statements: &Vec<Statement>) -> Result<Box<dyn Object>, EvalError> {
  let mut result: Result<Box<dyn Object>, EvalError> = Ok(Box::new(Null {}));

  for statement in statements {
    result = eval(statement);
  }

  result
}
