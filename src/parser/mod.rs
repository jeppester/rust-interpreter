#[cfg(test)]
mod tests;

use crate::lexer::Lexer;
use crate::token::Token;
use crate::ast::Program;

pub struct Parser<'a> {
  pub lexer: Lexer,
  pub current_token: Token<'a>,
  pub peek_token: Token<'a>,
}

impl<'a> Parser<'a> {
  pub fn new(mut lexer: Lexer) -> Self {
    let current_token = lexer.next_token();
    let peek_token = lexer.next_token();

    Parser {
      lexer: lexer,
      current_token: current_token,
      peek_token: peek_token,
    }
  }

  pub fn next_token(&mut self) {
    use std::mem;
    mem::swap(&mut self.current_token, &mut self.peek_token);
    self.peek_token = self.lexer.next_token();
  }

  pub fn parse_program(&mut self) -> Program {
    Program {
      statements: vec![],
    }
  }
}
