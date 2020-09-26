// use crate::ast::Expression;
use crate::ast::identifier::Identifier;
use crate::ast::Expression;
use crate::token::*;

#[derive(Debug, Clone)]
pub struct LetStatement {
  pub token: Token,
  pub name: Identifier,
  pub value: Expression,
}

impl LetStatement {
  pub fn token_literal(&self) -> Literal {
    self.token.literal.clone()
  }

  pub fn to_string(&self) -> String {
    let mut string = String::new();

    string.push_str(&self.token_literal());
    string.push(' ');
    string.push_str(&self.name.value);
    string.push_str(" = ");

    string.push_str(&self.value.to_string());

    string.push(';');

    string
  }
}
