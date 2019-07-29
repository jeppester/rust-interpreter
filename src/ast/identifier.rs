use crate::token::Token;

pub struct Identifier<'a> {
  pub token: Token<'a>,
  pub value: String,
}
