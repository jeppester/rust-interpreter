mod parser_error;

#[cfg(test)]
mod tests;

use std::collections::HashMap;

use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::*;

use boolean_literal::BooleanLiteral;
use function_literal::FunctionLiteral;
use identifier::Identifier;
use if_expression::IfExpression;
use infix_expression::InfixExpression;
use integer_literal::IntegerLiteral;
use prefix_expression::PrefixExpression;

use block_statement::BlockStatement;
use let_statement::LetStatement;
use return_statement::ReturnStatement;

use parser_error::ParserError;

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

pub fn get_operator_precedence(token_type: TokenType) -> Precedence {
  match token_type {
    EQ => precedences::EQUALS,
    NOT_EQ => precedences::EQUALS,
    LT => precedences::LESS_OR_GREATER,
    GT => precedences::LESS_OR_GREATER,
    PLUS => precedences::SUM,
    MINUS => precedences::SUM,
    SLASH => precedences::PRODUCT,
    ASTERISK => precedences::PRODUCT,
    _x => precedences::LOWEST,
  }
}

pub struct Parser {
  pub lexer: Lexer,
  pub current_token: Token,
  pub peek_token: Token,
  pub errors: Vec<ParserError>,
  pub prefix_parser_functions:
    HashMap<TokenType, fn(&mut Parser) -> Result<Expression, ParserError>>,
  pub infix_parser_functions:
    HashMap<TokenType, fn(&mut Parser, Expression) -> Result<Expression, ParserError>>,
}

pub fn parse_boolean(parser: &mut Parser) -> Result<Expression, ParserError> {
  return Ok(Expression::BooleanLiteral(BooleanLiteral {
    token: parser.current_token.clone(),
    value: parser.current_token_is(token_types::TRUE),
  }));
}

pub fn parse_identifier(parser: &mut Parser) -> Result<Expression, ParserError> {
  let token = parser.current_token.clone();
  let value = token.literal.clone();

  return Ok(Expression::Identifier(Identifier {
    token: token,
    value: value,
  }));
}

pub fn parse_integer_literal(parser: &mut Parser) -> Result<Expression, ParserError> {
  let token = parser.current_token.clone();

  let value = token.literal.parse::<i64>()?;

  Ok(Expression::IntegerLiteral(IntegerLiteral {
    token: token,
    value: value,
  }))
}

pub fn parse_prefix_expression(parser: &mut Parser) -> Result<Expression, ParserError> {
  let token = parser.current_token.clone();
  let operator = token.literal.clone();

  parser.next_token();

  let expression = parser.parse_expression(precedences::PREFIX)?;

  Ok(Expression::PrefixExpression(PrefixExpression {
    token: token,
    operator: operator,
    right: Box::new(expression),
  }))
}

pub fn parse_infix_expression(
  parser: &mut Parser,
  left: Expression,
) -> Result<Expression, ParserError> {
  let token = parser.current_token.clone();
  let operator = token.literal.clone();

  let precedence = parser.current_precedence();
  parser.next_token();

  let right = parser.parse_expression(precedence)?;

  Ok(Expression::InfixExpression(InfixExpression {
    token: token,
    left: Box::new(left),
    operator: operator,
    right: Box::new(right),
  }))
}

pub fn parse_grouped_expression(parser: &mut Parser) -> Result<Expression, ParserError> {
  parser.next_token();

  let expression = parser.parse_expression(precedences::LOWEST);

  parser.expect_peek(token_types::RPAREN)?;
  expression
}

pub fn parse_if_expression(parser: &mut Parser) -> Result<Expression, ParserError> {
  let token = parser.current_token.clone();

  parser.expect_peek(token_types::LPAREN)?;
  parser.next_token();

  let condition = parser.parse_expression(precedences::LOWEST)?;

  parser.expect_peek(token_types::RPAREN)?;
  parser.expect_peek(token_types::LBRACE)?;

  let true_block = parser.parse_block_statement();

  let false_block_or_none = if parser.peek_token_is(token_types::ELSE) {
    parser.next_token();
    parser.expect_peek(token_types::LBRACE)?;

    Some(parser.parse_block_statement())
  } else {
    None
  };

  Ok(Expression::IfExpression(IfExpression {
    token: token,
    condition: Box::new(condition),
    true_block: Box::new(true_block),
    false_block_or_none: Box::new(false_block_or_none),
  }))
}

pub fn parse_function_literal(parser: &mut Parser) -> Result<Expression, ParserError> {
  let token = parser.current_token.clone();

  parser.expect_peek(token_types::LPAREN)?;

  let arguments = parser.parse_function_arguments()?;

  parser.expect_peek(token_types::LBRACE)?;

  let body = parser.parse_block_statement();

  Ok(Expression::FunctionLiteral(FunctionLiteral {
    token: token,
    arguments: arguments,
    body: Box::new(body),
  }))
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
    parser.register_prefix(token_types::TRUE, parse_boolean);
    parser.register_prefix(token_types::FALSE, parse_boolean);

    parser.register_prefix(token_types::MINUS, parse_prefix_expression);
    parser.register_prefix(token_types::BANG, parse_prefix_expression);
    parser.register_prefix(token_types::LPAREN, parse_grouped_expression);

    parser.register_prefix(token_types::IF, parse_if_expression);
    parser.register_prefix(token_types::FUNCTION, parse_function_literal);

    parser.register_infix(token_types::EQ, parse_infix_expression);
    parser.register_infix(token_types::NOT_EQ, parse_infix_expression);
    parser.register_infix(token_types::LT, parse_infix_expression);
    parser.register_infix(token_types::GT, parse_infix_expression);
    parser.register_infix(token_types::PLUS, parse_infix_expression);
    parser.register_infix(token_types::MINUS, parse_infix_expression);
    parser.register_infix(token_types::SLASH, parse_infix_expression);
    parser.register_infix(token_types::ASTERISK, parse_infix_expression);

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
      let statement_result = self.parse_statement();

      match statement_result {
        Ok(statement) => program.statements.push(statement),
        Err(error) => self.errors.push(error),
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

  pub fn parse_block_statement(&mut self) -> BlockStatement {
    let token = self.current_token.clone();
    let mut statements = vec![];

    self.next_token();

    while !self.current_token_is(RBRACE) && !self.current_token_is(EOF) {
      let statement_result = self.parse_statement();

      match statement_result {
        Ok(statement) => statements.push(statement),
        Err(error) => self.errors.push(error),
      }

      self.next_token();
    }

    BlockStatement {
      token: token,
      statements: statements,
    }
  }

  pub fn parse_statement(&mut self) -> Result<Statement, ParserError> {
    match self.current_token.token_type {
      LET => self.parse_let_statement(),
      RETURN => self.parse_return_statement(),
      _x => self.parse_expression_statement(),
    }
  }

  pub fn parse_let_statement(&mut self) -> Result<Statement, ParserError> {
    let token = self.current_token.clone();

    self.expect_peek(IDENT)?;

    let name_token = self.current_token.clone();
    let name_value = name_token.literal.clone();
    let name = Identifier {
      token: name_token,
      value: name_value,
    };

    self.expect_peek(ASSIGN)?;

    // FIXME: Let statements should have expressions
    while !self.current_token_is(SEMICOLON) {
      self.next_token()
    }

    Ok(Statement::LetStatement(LetStatement {
      token: token,
      name: name,
    }))
  }

  pub fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
    let token = self.current_token.clone();

    self.next_token();

    let expression = self.parse_expression(precedences::LOWEST)?;

    if self.peek_token_is(SEMICOLON) {
      self.next_token();
    }

    let return_statement = ReturnStatement {
      token: token,
      return_value: Box::new(expression),
    };

    Ok(Statement::ReturnStatement(return_statement))
  }

  pub fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
    let expression = self.parse_expression(precedences::LOWEST)?;

    if self.peek_token_is(SEMICOLON) {
      self.next_token();
    }

    Ok(Statement::Expression(expression))
  }

  pub fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, ParserError> {
    let prefix_parser_function = self.prefix_parser_function_for(&self.current_token.token_type)?;
    let mut expression = prefix_parser_function(self)?;

    while !self.peek_token_is(SEMICOLON) && precedence < self.peek_precedence() {
      let has_infix_operator = self
        .infix_parser_functions
        .contains_key(&self.peek_token.token_type);

      if !has_infix_operator {
        return Ok(expression);
      }

      self.next_token();

      let infix_parser_function = self
        .infix_parser_functions
        .get(&self.current_token.token_type)
        .unwrap();

      expression = infix_parser_function(self, expression)?;
    }

    Ok(expression)
  }

  pub fn parse_function_arguments(&mut self) -> Result<Vec<Identifier>, ParserError> {
    let mut identifiers: Vec<Identifier> = Vec::new();

    if self.peek_token_is(token_types::RPAREN) {
      self.next_token();
      return Ok(identifiers);
    }

    loop {
      self.expect_peek(token_types::IDENT)?;

      let identifier = Identifier {
        token: self.current_token.clone(),
        value: self.current_token.literal.clone(),
      };
      identifiers.push(identifier);

      if self.peek_token_is(token_types::RPAREN) {
        self.next_token();
        break;
      };

      self.expect_peek(token_types::COMMA)?;
    }

    Ok(identifiers)
  }

  pub fn current_token_is(&mut self, token_type: TokenType) -> bool {
    self.current_token.token_type == token_type
  }

  pub fn peek_token_is(&mut self, token_type: TokenType) -> bool {
    self.peek_token.token_type == token_type
  }

  pub fn expect_peek(&mut self, token_type: TokenType) -> Result<(), ParserError> {
    if self.peek_token_is(token_type) {
      self.next_token();
      Ok(())
    } else {
      let error = ParserError(format!(
        "expected next token to be {}, got {} instead",
        token_type, self.peek_token.token_type
      ));

      Err(error)
    }
  }

  pub fn current_precedence(&mut self) -> Precedence {
    get_operator_precedence(self.current_token.token_type)
  }

  pub fn peek_precedence(&mut self) -> Precedence {
    get_operator_precedence(self.peek_token.token_type)
  }

  pub fn prefix_parser_function_for(
    &mut self,
    token_type: TokenType,
  ) -> Result<&fn(&mut Parser) -> Result<Expression, ParserError>, ParserError> {
    if let Some(parser_function) = self.prefix_parser_functions.get(token_type) {
      Ok(parser_function)
    } else {
      let error = ParserError(format!("no prefix parse function found for {}", token_type));
      Err(error)
    }
  }

  pub fn register_prefix(
    &mut self,
    token_type: TokenType,
    parser_function: fn(&mut Parser) -> Result<Expression, ParserError>,
  ) {
    self
      .prefix_parser_functions
      .insert(token_type, parser_function);
  }

  pub fn register_infix(
    &mut self,
    token_type: TokenType,
    parser_function: fn(&mut Parser, Expression) -> Result<Expression, ParserError>,
  ) {
    self
      .infix_parser_functions
      .insert(token_type, parser_function);
  }
}
