use crate::token::Token;

#[derive(Debug)]
pub struct BooleanLiteral {
  pub token: Token,
  pub value: bool,
}
