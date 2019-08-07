#[cfg(test)]
mod tests;

use crate::lexer::Lexer;
use crate::token::*;
use token_types::*;
use crate::ast::*;
use identifier::Identifier;
use let_statement::LetStatement;

pub struct Parser {
  pub lexer: Lexer,
  pub current_token: Token,
  pub peek_token: Token,
  pub errors: Vec<String>,
}

impl Parser {
  pub fn new(mut lexer: Lexer) -> Self {
    let current_token = lexer.next_token();
    let peek_token = lexer.next_token();

    Parser {
      lexer: lexer,
      current_token: current_token,
      peek_token: peek_token,
      errors: vec![],
    }
  }

  pub fn next_token(&mut self) {
    use std::mem;
    mem::swap(&mut self.current_token, &mut self.peek_token);
    self.peek_token = self.lexer.next_token();
  }

  pub fn parse_program(&mut self) -> Program {
    let mut program = Program { statements: vec![] };

    while !self.current_token_is(EOF) {
      let statement_or_none = self.parse_statement();

      if let Some(statement) = statement_or_none {
        program.statements.push(statement);
      }

      self.next_token();
    }

    let error_count = self.errors.len();
    if error_count == 0 {
      program
    }
    else {
      println!("Parser has {} error(s):", error_count);

      for error in &self.errors {
        println!("parser error: {}", error);
      }

      panic!();
    }
  }

  pub fn parse_statement(&mut self) -> Option<Node> {
    match self.current_token.token_type {
      LET => self.parse_let_statement(),
      _x => None,
    }
  }

  pub fn parse_let_statement(&mut self) -> Option<Node> {
    let token = self.current_token.clone();

    if !self.expect_peek(IDENT) {
      return None
    }

    let name_token = self.current_token.clone();
    let name_value = name_token.literal.clone().unwrap();
    let name = Identifier { token: name_token, value: name_value };

    if !self.expect_peek(ASSIGN) {
      return None
    }

    while !self.current_token_is(SEMICOLON) {
      self.next_token()
    }

    Some(Node::Statement(Statement::LetStatement(LetStatement {
      token: token,
      name: name,
    })))
  }

  pub fn current_token_is(&mut self, token_type: TokenType) -> bool {
    self.current_token.token_type == token_type
  }

  pub fn peek_token_is(&mut self, token_type: TokenType) -> bool {
    self.peek_token.token_type == token_type
  }

  pub fn expect_peek(&mut self, token_type: TokenType) -> bool {
    if self.peek_token_is(token_type) {
      self.next_token();
      true
    }
    else {
      self.peek_error(token_type);
      false
    }
  }

  pub fn peek_error(&mut self, token_type: TokenType) {
    let error = format!("expected next token to be {}, got {} instead", token_type, self.peek_token.token_type);
    self.errors.push(error);
  }
}
