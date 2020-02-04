#[cfg(test)]
mod tests;

use std::collections::HashMap;

use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::*;
use identifier::Identifier;
use integer_literal::IntegerLiteral;
use prefix_expression::PrefixExpression;
use let_statement::LetStatement;
use return_statement::ReturnStatement;
use token_types::*;

pub type Precedence = u8;

pub mod precedences {
  pub const LOWEST: u8 = 0;
  pub const EQUALS: u8 = 1;
  pub const LESS_OR_GREATER: u8 = 2;
  pub const SUM: u8 = 3;
  pub const PRODUCT: u8 = 4;
  pub const PREFIX: u8 = 5;
  pub const CALL: u8 = 6;
}

pub struct Parser {
  pub lexer: Lexer,
  pub current_token: Token,
  pub peek_token: Token,
  pub errors: Vec<String>,
  pub prefix_parser_functions: HashMap<TokenType, fn(&mut Parser) -> Option<Expression>>,
  pub infix_parser_functions: HashMap<TokenType, fn(&mut Parser, Expression) -> Option<Expression>>,
}

pub fn parse_identifier(parser: &mut Parser) -> Option<Expression> {
  let token = parser.current_token.clone();
  let value = token.literal.clone();

  return Some(Expression::Identifier(Identifier {
    token: token,
    value: value,
  }));
}

pub fn parse_integer_literal(parser: &mut Parser) -> Option<Expression> {
  let token = parser.current_token.clone();

  match token.literal.parse::<i64>() {
    Ok(value) => Some(Expression::IntegerLiteral(IntegerLiteral {
      token: token,
      value: value,
    })),
    Err(_error) => {
      let error = format!(
        "could not parse {} as integer",
        token.literal
      );
      parser.errors.push(error);
      None
    },
  }
}

pub fn parse_prefix_expression(parser: &mut Parser) -> Option<Expression> {
  let token = parser.current_token.clone();
  let operator = token.literal.clone();

  parser.next_token();

  let expression_or_none = parser.parse_expression(precedences::PREFIX);

  match expression_or_none {
    Some(expression) => {
      Some(Expression::PrefixExpression(PrefixExpression {
        token: token,
        operator: operator,
        right: Box::new(expression),
      }))
    },
    None => {
      parser.no_right_error(token.token_type);
      None
    }
  }
}

impl Parser {
  pub fn new(mut lexer: Lexer) -> Self {
    let current_token = lexer.next_token();
    let peek_token = lexer.next_token();

    let mut parser = Parser {
      lexer: lexer,
      current_token: current_token,
      peek_token: peek_token,
      errors: vec![],
      prefix_parser_functions: HashMap::new(),
      infix_parser_functions: HashMap::new(),
    };

    parser.register_prefix(token_types::IDENT, parse_identifier);
    parser.register_prefix(token_types::INT, parse_integer_literal);
    parser.register_prefix(token_types::MINUS, parse_prefix_expression);
    parser.register_prefix(token_types::BANG, parse_prefix_expression);

    parser
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
    } else {
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
      RETURN => self.parse_return_statement(),
      _x => self.parse_expression_statement(),
    }
  }

  pub fn parse_let_statement(&mut self) -> Option<Node> {
    let token = self.current_token.clone();

    if !self.expect_peek(IDENT) {
      return None;
    }

    let name_token = self.current_token.clone();
    let name_value = name_token.literal.clone();
    let name = Identifier {
      token: name_token,
      value: name_value,
    };

    if !self.expect_peek(ASSIGN) {
      return None;
    }

    while !self.current_token_is(SEMICOLON) {
      self.next_token()
    }

    Some(Node::Statement(Statement::LetStatement(LetStatement {
      token: token,
      name: name,
    })))
  }

  pub fn parse_return_statement(&mut self) -> Option<Node> {
    let token = self.current_token.clone();

    while !self.current_token_is(SEMICOLON) {
      self.next_token()
    }

    Some(Node::Statement(Statement::ReturnStatement(
      ReturnStatement { token: token },
    )))
  }

  pub fn parse_expression_statement(&mut self) -> Option<Node> {
    let token = self.current_token.clone();
    let expression_or_none = self.parse_expression(precedences::LOWEST);

    if let Some(expression) = expression_or_none {
      if self.peek_token_is(SEMICOLON) {
        self.next_token();
      }

      Some(Node::Expression(expression))
    } else {
      None
    }
  }

  pub fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
    let parser_function_or_none = self
      .prefix_parser_functions
      .get(&self.current_token.token_type);

    if let Some(parser_function) = parser_function_or_none {
      parser_function(self)
    } else {
      self.prefix_parser_error(self.current_token.token_type);
      None
    }
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
    } else {
      self.peek_error(token_type);
      false
    }
  }

  pub fn prefix_parser_error(&mut self, token_type: TokenType) {
    let error = format!("no prefix parse function found for {}", token_type);
    self.errors.push(error);
  }

  pub fn no_right_error(&mut self, token_type: TokenType) {
    let error = format!("no right side found for prefix expression {}", token_type);
    self.errors.push(error);
  }

  pub fn peek_error(&mut self, token_type: TokenType) {
    let error = format!(
      "expected next token to be {}, got {} instead",
      token_type, self.peek_token.token_type
    );
    self.errors.push(error);
  }

  pub fn register_prefix(
    &mut self,
    token_type: TokenType,
    parser_function: fn(&mut Parser) -> Option<Expression>,
  ) {
    self
      .prefix_parser_functions
      .insert(token_type, parser_function);
  }

  pub fn register_infix(
    &mut self,
    token_type: TokenType,
    parser_function: fn(&mut Parser, Expression) -> Option<Expression>,
  ) {
    self
      .infix_parser_functions
      .insert(token_type, parser_function);
  }
}
