use crate::token::*;

#[derive(Debug)]
pub struct BooleanLiteral {
  pub token: Token,
  pub value: bool,
}

impl BooleanLiteral {
  pub fn token_literal(&self) -> Literal {
    self.token.literal.clone()
  }

  pub fn to_string(&self) -> String {
    self.value.to_string()
  }
}
