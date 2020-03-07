#[cfg(test)]
mod tests;

pub mod boolean;
pub mod identifier;
pub mod infix_expression;
pub mod integer_literal;
pub mod let_statement;
pub mod prefix_expression;
pub mod return_statement;

use crate::token::Literal;
use boolean::Boolean;
use identifier::Identifier;
use infix_expression::InfixExpression;
use integer_literal::IntegerLiteral;
use let_statement::LetStatement;
use prefix_expression::PrefixExpression;
use return_statement::ReturnStatement;

#[derive(Debug)]
pub enum Expression {
  Boolean(Boolean),
  Identifier(Identifier),
  IntegerLiteral(IntegerLiteral),
  PrefixExpression(PrefixExpression),
  InfixExpression(InfixExpression),
}

impl Expression {
  pub fn token_literal(&self) -> Literal {
    match self {
      Expression::Identifier(expression) => expression.token.literal.clone(),
      Expression::Boolean(boolean) => boolean.token.literal.clone(),
      Expression::IntegerLiteral(integer_literal) => integer_literal.token.literal.clone(),
      Expression::PrefixExpression(prefix_expression) => prefix_expression.token.literal.clone(),
      Expression::InfixExpression(infix_expression) => infix_expression.token.literal.clone(),
    }
  }

  pub fn to_string(&self) -> String {
    match self {
      Expression::Identifier(expression) => expression.value.clone(),
      Expression::IntegerLiteral(integer_literal) => integer_literal.value.to_string(),
      Expression::Boolean(boolean) => boolean.value.to_string(),
      Expression::PrefixExpression(prefix_expression) => {
        let mut string = String::new();

        string.push('(');
        string.push_str(&prefix_expression.operator.to_string());
        string.push_str(&prefix_expression.right.to_string());
        string.push(')');

        string
      }
      Expression::InfixExpression(infix_expression) => {
        let mut string = String::new();

        string.push('(');
        string.push_str(&infix_expression.left.to_string());
        string.push(' ');
        string.push_str(&infix_expression.operator.to_string());
        string.push(' ');
        string.push_str(&infix_expression.right.to_string());
        string.push(')');

        string
      }
    }
  }
}

#[derive(Debug)]
pub enum Statement {
  LetStatement(LetStatement),
  ReturnStatement(ReturnStatement),
  Expression(Expression),
}

impl Statement {
  pub fn token_literal(&self) -> Literal {
    match self {
      Statement::LetStatement(statement) => statement.token.literal.clone(),
      Statement::ReturnStatement(statement) => statement.token.literal.clone(),
      Statement::Expression(expression) => expression.token_literal(),
    }
  }

  pub fn to_string(&self) -> String {
    match self {
      Statement::LetStatement(statement) => {
        let mut string = String::new();

        string.push_str(&self.token_literal());
        string.push(' ');
        string.push_str(&statement.name.value);
        string.push_str(" = ");

        string.push_str("[TODO: EXPRESSION]");

        string.push(';');

        string
      }

      Statement::ReturnStatement(_statement) => {
        let mut string = String::new();

        string.push_str(&self.token_literal());
        string.push(' ');

        string.push_str("[TODO: RETURN VALUE]");

        string.push(';');

        string
      }

      Statement::Expression(expression) => expression.to_string(),
    }
  }
}

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
