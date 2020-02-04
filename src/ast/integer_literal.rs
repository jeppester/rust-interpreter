use crate::token::Token;

#[derive(Debug)]
pub struct IntegerLiteral {
  pub token: Token,
  pub value: i64,
}
