use crate::ast::BlockStatement;
use crate::ast::Expression;
use crate::token::Token;

#[derive(Debug)]
pub struct IfExpression {
  pub token: Token,
  pub condition: Box<Expression>,
  pub true_block: Box<BlockStatement>,
  pub false_block_or_none: Box<Option<BlockStatement>>,
}
