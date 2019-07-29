pub mod let_statement;
pub mod identifier;

use identifier::Identifier;
use let_statement::LetStatement;
use crate::token::Literal;

pub enum Expression<'a> {
  Identifier(Identifier<'a>),
}

impl<'a> Expression<'a> {
  pub fn token_literal(&self) -> &Literal {
    match self {
      Expression::Identifier(expression) => &expression.token.literal,
    }
  }
}

pub enum Statement<'a> {
  LetStatement(LetStatement<'a>),
}

impl<'a> Statement<'a> {
  pub fn token_literal(&self) -> &Literal {
    match self {
      Statement::LetStatement(statement) => &statement.token.literal,
    }
  }
}

pub enum Node<'a> {
  Expression(Expression<'a>),
  Statement(Statement<'a>),
}

impl<'a> Node<'a> {
  pub fn token_literal(&self) -> &Literal {
    match self {
      Node::Expression(expression) => expression.token_literal(),
      Node::Statement(statement) => statement.token_literal(),
    }
  }
}

pub struct Program<'a> {
  pub statements: Vec<Node <'a>>,
}

impl<'a> Program<'a> {
  pub fn token_literal(&self) -> &Literal {
    if self.statements.len() > 0 {
      self.statements[0].token_literal()
    }
    else {
      panic!("Empty program!");
    }
  }
}
