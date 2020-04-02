use crate::ast::BlockStatement;
use crate::ast::Identifier;
use crate::token::*;

#[derive(Debug)]
pub struct FunctionLiteral {
  pub token: Token,
  pub arguments: Vec<Identifier>,
  pub body: Box<BlockStatement>,
}

impl FunctionLiteral {
  pub fn token_literal(&self) -> Literal {
    self.token.literal.clone()
  }

  pub fn to_string(&self) -> String {
    let mut string = String::new();

    string.push_str("fn ");
    string.push('(');
    string.push_str(
      &self
        .arguments
        .iter()
        .map(|identifier| identifier.to_string())
        .collect::<Vec<String>>()
        .join(", "),
    );
    string.push_str(") ");
    string.push_str(&*self.body.to_string());

    string
  }
}
