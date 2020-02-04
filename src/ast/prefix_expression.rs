use crate::token::Token;
use crate::ast::Expression;

#[derive(Debug)]
pub struct PrefixExpression {
  pub token: Token,
  pub operator: String,
  pub right: Box<Expression>,
}
