pub mod let_statement;
pub mod identifier;

use identifier::Identifier;
use let_statement::LetStatement;
use crate::token::Literal;

pub enum Expression {
  Identifier(Identifier),
}

impl Expression {
  pub fn token_literal(&self) -> &Literal {
    match self {
      Expression::Identifier(expression) => &expression.token.literal,
    }
  }
}

pub enum Statement {
  LetStatement(LetStatement),
}

impl Statement {
  pub fn token_literal(&self) -> &Literal {
    match self {
      Statement::LetStatement(statement) => &statement.token.literal,
    }
  }
}

pub enum Node {
  Expression(Expression),
  Statement(Statement),
}

impl Node {
  pub fn token_literal(&self) -> &Literal {
    match self {
      Node::Expression(expression) => expression.token_literal(),
      Node::Statement(statement) => statement.token_literal(),
    }
  }
}

pub struct Program {
  pub statements: Vec<Node>,
}

impl Program {
  pub fn token_literal(&self) -> &Literal {
    if self.statements.len() > 0 {
      self.statements[0].token_literal()
    }
    else {
      panic!("Empty program!");
    }
  }
}
