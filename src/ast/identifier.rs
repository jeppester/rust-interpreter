use crate::token::*;

#[derive(Debug)]
pub struct Identifier {
  pub token: Token,
  pub value: String,
}

impl Identifier {
  pub fn token_literal(&self) -> Literal {
    self.token.literal.clone()
  }

  pub fn to_string(&self) -> String {
    self.value.clone()
  }
}
