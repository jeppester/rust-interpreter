use crate::ast::BlockStatement;
use crate::ast::Expression;
use crate::token::*;

#[derive(Debug)]
pub struct IfExpression {
  pub token: Token,
  pub condition: Box<Expression>,
  pub true_block: Box<BlockStatement>,
  pub false_block_or_none: Box<Option<BlockStatement>>,
}

impl IfExpression {
  pub fn token_literal(&self) -> Literal {
    self.token.literal.clone()
  }

  pub fn to_string(&self) -> String {
    let mut string = String::new();

    string.push_str("if ");
    string.push('(');
    string.push_str(&self.condition.to_string());
    string.push_str(") {\n");
    string.push_str(&*self.true_block.to_string());

    if let Some(false_block) = &*self.false_block_or_none {
      string.push_str("} else {\n");
      string.push_str(&*false_block.to_string());
    } else {
      string.push('}');
    }

    string
  }
}
