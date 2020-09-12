#[cfg(test)]
mod tests;

pub mod block_statement;
pub mod boolean_literal;
pub mod call_expression;
pub mod function_literal;
pub mod identifier;
pub mod if_expression;
pub mod infix_expression;
pub mod integer_literal;
pub mod let_statement;
pub mod prefix_expression;
pub mod return_statement;

use crate::token::Literal;
use block_statement::BlockStatement;
use boolean_literal::BooleanLiteral;
use call_expression::CallExpression;
use function_literal::FunctionLiteral;
use identifier::Identifier;
use if_expression::IfExpression;
use infix_expression::InfixExpression;
use integer_literal::IntegerLiteral;
use let_statement::LetStatement;
use prefix_expression::PrefixExpression;
use return_statement::ReturnStatement;

#[derive(Debug)]
pub enum Expression {
  BooleanLiteral(BooleanLiteral),
  Identifier(Identifier),
  IntegerLiteral(IntegerLiteral),
  PrefixExpression(PrefixExpression),
  InfixExpression(InfixExpression),
  IfExpression(IfExpression),
  FunctionLiteral(FunctionLiteral),
  CallExpression(CallExpression),
}

impl Expression {
  pub fn token_literal(&self) -> Literal {
    match self {
      Expression::Identifier(expression) => expression.token_literal(),
      Expression::BooleanLiteral(boolean_literal) => boolean_literal.token_literal(),
      Expression::IntegerLiteral(integer_literal) => integer_literal.token_literal(),
      Expression::PrefixExpression(prefix_expression) => prefix_expression.token_literal(),
      Expression::InfixExpression(infix_expression) => infix_expression.token_literal(),
      Expression::IfExpression(if_expression) => if_expression.token_literal(),
      Expression::FunctionLiteral(function_literal) => function_literal.token_literal(),
      Expression::CallExpression(call_expression) => call_expression.token_literal(),
    }
  }

  pub fn to_string(&self) -> String {
    match self {
      Expression::Identifier(expression) => expression.to_string(),
      Expression::IntegerLiteral(integer_literal) => integer_literal.to_string(),
      Expression::BooleanLiteral(boolean_literal) => boolean_literal.to_string(),
      Expression::PrefixExpression(prefix_expression) => prefix_expression.to_string(),
      Expression::InfixExpression(infix_expression) => infix_expression.to_string(),
      Expression::IfExpression(if_expression) => if_expression.to_string(),
      Expression::FunctionLiteral(function_literal) => function_literal.to_string(),
      Expression::CallExpression(call_expression) => call_expression.to_string(),
    }
  }
}

#[derive(Debug)]
pub enum Statement {
  LetStatement(LetStatement),
  ReturnStatement(ReturnStatement),
  Expression(Expression),
  BlockStatement(BlockStatement),
}

impl Statement {
  pub fn token_literal(&self) -> Literal {
    match self {
      Statement::LetStatement(let_statement) => let_statement.token_literal(),
      Statement::ReturnStatement(return_statement) => return_statement.token_literal(),
      Statement::Expression(expression) => expression.token_literal(),
      Statement::BlockStatement(block_statement) => block_statement.token_literal(),
    }
  }

  pub fn to_string(&self) -> String {
    match self {
      Statement::LetStatement(let_statement) => let_statement.to_string(),
      Statement::ReturnStatement(return_statement) => return_statement.to_string(),
      Statement::Expression(expression) => expression.to_string(),
      Statement::BlockStatement(block_statement) => block_statement.to_string(),
    }
  }
}

#[derive(Debug)]
pub struct Program {
  pub statements: Vec<Statement>,
}

impl Program {
  pub fn token_literal(&self) -> Literal {
    if self.statements.len() > 0 {
      self.statements[0].token_literal()
    } else {
      panic!("Empty program!");
    }
  }

  pub fn to_string(&self) -> String {
    let mut string = String::new();

    for statement in &self.statements {
      string.push_str(&statement.to_string());
    }

    string
  }
}
