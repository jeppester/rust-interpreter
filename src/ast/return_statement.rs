use crate::ast::Expression;
use crate::token::Token;

#[derive(Debug)]
pub struct ReturnStatement {
  pub token: Token,
  pub return_value: Box<Expression>,
}
