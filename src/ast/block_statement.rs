use crate::ast::Statement;
use crate::token::*;

#[derive(Debug, Clone)]
pub struct BlockStatement {
  pub token: Token,
  pub statements: Vec<Statement>,
}

impl BlockStatement {
  pub fn token_literal(&self) -> Literal {
    self.token.literal.clone()
  }

  pub fn to_string(&self) -> String {
    let mut string = String::new();

    string.push('{');
    string.push('\n');

    for statement in &self.statements {
      string.push_str(&statement.to_string());
      string.push('\n');
    }

    string.push('}');
    string.push('\n');

    string
  }
}
