use crate::ast::Expression;
use crate::token::Token;

pub struct ExpressionStatement {
  pub token: Token,
  pub expression: Expression,
}
