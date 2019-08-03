// use crate::ast::Expression;
use crate::ast::identifier::Identifier;
use crate::token::Token;

pub struct LetStatement {
  pub token: Token,
  pub name: Identifier,
  // pub value: Expression,
}
