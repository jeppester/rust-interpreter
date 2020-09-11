#[cfg(test)]
mod tests;

use crate::ast::*;
use crate::object::*;

// use boolean_literal::BooleanLiteral;
// use call_expression::CallExpression;
// use function_literal::FunctionLiteral;
// use identifier::Identifier;
// use if_expression::IfExpression;
// use infix_expression::InfixExpression;
use integer_literal::IntegerLiteral;
// use prefix_expression::PrefixExpression;
// use block_statement::BlockStatement;
// use let_statement::LetStatement;
// use return_statement::ReturnStatement;


pub trait EvalObject {
  fn eval(&self) -> Box<dyn Object>;
}

impl EvalObject for Program {
  fn eval(&self) -> Box<dyn Object> {
    eval_statements(&self.statements)
  }
}

impl EvalObject for Statement {
  fn eval(&self) -> Box<dyn Object> {
    match &self {
      Statement::LetStatement(_) => Box::new(Null {}),
      Statement::ReturnStatement(_) => Box::new(Null {}),
      Statement::Expression(expression) => expression.eval(),
      Statement::BlockStatement(_) => Box::new(Null {}),
    }
  }
}

impl EvalObject for Expression {
  fn eval(&self) -> Box<dyn Object> {
    match &self {
      Expression::Identifier(expression) => Box::new(Null {}),
      Expression::BooleanLiteral(boolean_literal) => Box::new(Null {}),
      Expression::IntegerLiteral(integer_literal) => integer_literal.eval(),
      Expression::PrefixExpression(prefix_expression) => Box::new(Null {}),
      Expression::InfixExpression(infix_expression) => Box::new(Null {}),
      Expression::IfExpression(if_expression) => Box::new(Null {}),
      Expression::FunctionLiteral(function_literal) => Box::new(Null {}),
      Expression::CallExpression(call_expression) => Box::new(Null {}),
    }
  }
}

impl EvalObject for IntegerLiteral {
  fn eval(&self) -> Box<dyn Object> {
    Box::new(Integer { value: self.value.clone() })
  }
}

pub fn eval(node: &impl EvalObject) -> Box<dyn Object> {
  node.eval()
}

pub fn eval_statements(statements: &Vec<Statement>) -> Box<dyn Object> {
  let mut result: Box<dyn Object> = Box::new(Null {});

  for statement in statements {
    result = eval(statement);
  }

  result
}
