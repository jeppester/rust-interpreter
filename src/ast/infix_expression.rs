use crate::ast::Expression;
use crate::token::*;

#[derive(Debug)]
pub struct InfixExpression {
  pub token: Token,
  pub left: Box<Expression>,
  pub operator: String,
  pub right: Box<Expression>,
}

impl InfixExpression {
  pub fn token_literal(&self) -> Literal {
    self.token.literal.clone()
  }

  pub fn to_string(&self) -> String {
    let mut string = String::new();

    string.push('(');
    string.push_str(&self.left.to_string());
    string.push(' ');
    string.push_str(&self.operator.to_string());
    string.push(' ');
    string.push_str(&self.right.to_string());
    string.push(')');

    string
  }
}
