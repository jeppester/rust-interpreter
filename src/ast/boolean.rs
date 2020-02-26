use crate::token::Token;

#[derive(Debug)]
pub struct Boolean {
  pub token: Token,
  pub value: bool,
}
