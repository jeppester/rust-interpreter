use crate::ast::Expression;
use crate::token::*;

#[derive(Debug)]
pub struct CallExpression {
  pub token: Token,
  pub function: Box<Expression>,
  pub arguments: Box<Vec<Expression>>,
}

impl CallExpression {
  pub fn token_literal(&self) -> Literal {
    self.token.literal.clone()
  }

  pub fn to_string(&self) -> String {
    let mut string = String::new();

    string.push_str(&*self.function.to_string());
    string.push('(');

    string.push_str(
      &self
        .arguments
        .iter()
        .map(|argument| argument.to_string())
        .collect::<Vec<String>>()
        .join(", "),
    );

    string.push(')');
    string
  }
}
