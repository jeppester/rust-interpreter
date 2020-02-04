use crate::ast::Expression;
use crate::token::Token;

#[derive(Debug)]
pub struct ExpressionStatement {
  pub token: Token,
  pub expression: Expression,
}
