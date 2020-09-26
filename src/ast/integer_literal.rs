use crate::token::*;

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
  pub token: Token,
  pub value: i64,
}

impl IntegerLiteral {
  pub fn token_literal(&self) -> Literal {
    self.token.literal.clone()
  }

  pub fn to_string(&self) -> String {
    self.value.to_string()
  }
}
