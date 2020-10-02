use crate::token::*;

#[derive(Debug, Clone)]
pub struct StringLiteral {
  pub token: Token,
  pub value: String,
}

impl StringLiteral {
  pub fn token_literal(&self) -> Literal {
    self.token.literal.clone()
  }

  pub fn to_string(&self) -> String {
    self.value.clone()
  }
}
