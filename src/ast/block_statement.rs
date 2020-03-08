use crate::ast::Statement;
use crate::token::Token;

#[derive(Debug)]
pub struct BlockStatement {
  pub token: Token,
  pub statements: Vec<Statement>,
}

impl BlockStatement {
  pub fn to_string(&self) -> String {
    let mut string = String::new();

    for statement in &self.statements {
      string.push_str(&statement.to_string());
    }

    string
  }
}
