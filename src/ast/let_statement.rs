use crate::ast::Expression;
use crate::ast::identifier::Identifier;
use crate::token::Token;

pub struct LetStatement<'a> {
  pub token: Token<'a>,
  pub name: Identifier<'a>,
  pub value: Expression<'a>,
}
