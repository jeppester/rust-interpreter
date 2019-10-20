#[cfg(test)]
mod tests;

pub mod let_statement;
pub mod return_statement;
pub mod identifier;
pub mod expression_statement;

use identifier::Identifier;
use let_statement::LetStatement;
use return_statement::ReturnStatement;
use expression_statement::ExpressionStatement;
use crate::token::Literal;

pub enum Expression {
  Identifier(Identifier),
}

impl Expression {
  pub fn token_literal(&self) -> Literal {
    match self {
      Expression::Identifier(expression) => expression.token.literal.clone(),
    }
  }

  pub fn to_string(&self) -> String {
    match self {
      Expression::Identifier(expression) => expression.value.clone(),
    }
  }
}

pub enum Statement {
  LetStatement(LetStatement),
  ReturnStatement(ReturnStatement),
  ExpressionStatement(ExpressionStatement),
}

impl Statement {
  pub fn token_literal(&self) -> Literal {
    match self {
      Statement::LetStatement(statement) => statement.token.literal.clone(),
      Statement::ReturnStatement(statement) => statement.token.literal.clone(),
      Statement::ExpressionStatement(statement) => statement.token.literal.clone(),
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
      },

      Statement::ReturnStatement(_statement) => {
        let mut string = String::new();

        string.push_str(&self.token_literal());
        string.push(' ');

        string.push_str("[TODO: RETURN VALUE]");

        string.push(';');

        string
      },

      Statement::ExpressionStatement(statement) => {
        statement.expression.to_string()
      },
    }
  }
}

pub enum Node {
  Expression(Expression),
  Statement(Statement),
}

impl Node {
  pub fn token_literal(&self) -> Literal {
    match self {
      Node::Expression(expression) => expression.token_literal(),
      Node::Statement(statement) => statement.token_literal(),
    }
  }

  pub fn to_string(&self) -> String {
    match self {
      Node::Expression(expression) => expression.to_string(),
      Node::Statement(statement) => statement.to_string(),
    }
  }
}

pub struct Program {
  pub statements: Vec<Node>,
}

impl Program {
  pub fn token_literal(&self) -> Literal {
    if self.statements.len() > 0 {
      self.statements[0].token_literal()
    }
    else {
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
