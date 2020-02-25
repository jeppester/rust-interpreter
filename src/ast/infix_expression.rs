use crate::token::Token;
use crate::ast::Expression;

#[derive(Debug)]
pub struct InfixExpression {
  pub token: Token,
  pub left: Box<Expression>,
  pub operator: String,
  pub right: Box<Expression>,
}
