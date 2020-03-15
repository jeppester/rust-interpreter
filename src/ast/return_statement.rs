use crate::ast::Expression;
use crate::token::*;

#[derive(Debug)]
pub struct ReturnStatement {
  pub token: Token,
  pub return_value: Box<Expression>,
}

impl ReturnStatement {
  pub fn token_literal(&self) -> Literal {
    self.token.literal.clone()
  }

  pub fn to_string(&self) -> String {
    let mut string = String::new();

    string.push_str(&self.token_literal());
    string.push(' ');

    string.push_str("[TODO: RETURN VALUE]");

    string.push(';');

    string
  }
}
