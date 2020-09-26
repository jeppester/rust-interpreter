use crate::ast::Expression;
use crate::token::*;

#[derive(Debug, Clone)]
pub struct PrefixExpression {
  pub token: Token,
  pub operator: String,
  pub right: Box<Expression>,
}

impl PrefixExpression {
  pub fn token_literal(&self) -> Literal {
    self.token.literal.clone()
  }

  pub fn to_string(&self) -> String {
    let mut string = String::new();

    string.push('(');
    string.push_str(&self.operator.to_string());
    string.push_str(&self.right.to_string());
    string.push(')');

    string
  }
}
