#[cfg(test)]
mod tests;

use std::collections::HashMap;

use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::*;

use boolean_literal::BooleanLiteral;
use identifier::Identifier;
use if_expression::IfExpression;
use infix_expression::InfixExpression;
use integer_literal::IntegerLiteral;
use prefix_expression::PrefixExpression;

use block_statement::BlockStatement;
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
  pub errors: Vec<String>,
  pub prefix_parser_functions: HashMap<TokenType, fn(&mut Parser) -> Option<Expression>>,
  pub infix_parser_functions: HashMap<TokenType, fn(&mut Parser, Expression) -> Option<Expression>>,
}

pub fn parse_boolean(parser: &mut Parser) -> Option<Expression> {
  return Some(Expression::BooleanLiteral(BooleanLiteral {
    token: parser.current_token.clone(),
    value: parser.current_token_is(token_types::TRUE),
  }));
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
      let error = format!("could not parse {} as integer", token.literal);
      parser.errors.push(error);
      None
    }
  }
}

pub fn parse_prefix_expression(parser: &mut Parser) -> Option<Expression> {
  let token = parser.current_token.clone();
  let operator = token.literal.clone();

  parser.next_token();

  let expression_or_none = parser.parse_expression(precedences::PREFIX);

  match expression_or_none {
    Some(expression) => Some(Expression::PrefixExpression(PrefixExpression {
      token: token,
      operator: operator,
      right: Box::new(expression),
    })),
    None => {
      parser.no_right_error(token.token_type);
      None
    }
  }
}

pub fn parse_infix_expression(parser: &mut Parser, left: Expression) -> Option<Expression> {
  let token = parser.current_token.clone();
  let operator = token.literal.clone();

  let precedence = parser.current_precedence();
  parser.next_token();

  let right_or_none = parser.parse_expression(precedence);

  match right_or_none {
    Some(right) => Some(Expression::InfixExpression(InfixExpression {
      token: token,
      left: Box::new(left),
      operator: operator,
      right: Box::new(right),
    })),
    None => {
      parser.no_right_error(token.token_type);
      None
    }
  }
}

pub fn parse_grouped_expression(parser: &mut Parser) -> Option<Expression> {
  parser.next_token();

  let expression = parser.parse_expression(precedences::LOWEST);

  if !parser.expect_peek(token_types::RPAREN) {
    None
  } else {
    expression
  }
}

pub fn parse_if_expression(parser: &mut Parser) -> Option<Expression> {
  let token = parser.current_token.clone();

  if !parser.expect_peek(token_types::LPAREN) {
    return None;
  }

  parser.next_token();

  let condition_or_none = parser.parse_expression(precedences::LOWEST);

  if let None = condition_or_none {
    return None;
  }

  let condition = condition_or_none.unwrap();

  if !parser.expect_peek(token_types::RPAREN) || !parser.expect_peek(token_types::LBRACE) {
    return None;
  }

  let true_block = parser.parse_block_statement();

  let false_block_or_none = if parser.peek_token_is(token_types::ELSE) {
    parser.next_token();
    if !parser.expect_peek(token_types::LBRACE) {
      None
    } else {
      Some(parser.parse_block_statement())
    }
  } else {
    None
  };

  Some(Expression::IfExpression(IfExpression {
    token: token,
    condition: Box::new(condition),
    true_block: Box::new(true_block),
    false_block_or_none: Box::new(false_block_or_none),
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

  pub fn parse_block_statement(&mut self) -> BlockStatement {
    let token = self.current_token.clone();
    let mut statements = vec![];

    self.next_token();

    while !self.current_token_is(RBRACE) && !self.current_token_is(EOF) {
      let statement_or_none = self.parse_statement();

      if let Some(statement) = statement_or_none {
        statements.push(statement);
      }

      self.next_token();
    }

    BlockStatement {
      token: token,
      statements: statements,
    }
  }

  pub fn parse_statement(&mut self) -> Option<Statement> {
    match self.current_token.token_type {
      LET => self.parse_let_statement(),
      RETURN => self.parse_return_statement(),
      _x => self.parse_expression_statement(),
    }
  }

  pub fn parse_let_statement(&mut self) -> Option<Statement> {
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

    Some(Statement::LetStatement(LetStatement {
      token: token,
      name: name,
    }))
  }

  pub fn parse_return_statement(&mut self) -> Option<Statement> {
    let token = self.current_token.clone();

    self.next_token();

    let expression_or_none = self.parse_expression(precedences::LOWEST);

    if let Some(expression) = expression_or_none {
      if self.peek_token_is(SEMICOLON) {
        self.next_token();
      }

      let return_statement = ReturnStatement {
        token: token,
        return_value: Box::new(expression),
      };

      Some(Statement::ReturnStatement(return_statement))
    } else {
      None
    }
  }

  pub fn parse_expression_statement(&mut self) -> Option<Statement> {
    let expression_or_none = self.parse_expression(precedences::LOWEST);

    if let Some(expression) = expression_or_none {
      if self.peek_token_is(SEMICOLON) {
        self.next_token();
      }

      Some(Statement::Expression(expression))
    } else {
      None
    }
  }

  pub fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
    let prefix_parser_function_or_none = self
      .prefix_parser_functions
      .get(&self.current_token.token_type);

    if let Some(prefix_parser_function) = prefix_parser_function_or_none {
      let mut expression = prefix_parser_function(self);
      if let None = expression {
        return None;
      }

      while !self.peek_token_is(SEMICOLON) && precedence < self.peek_precedence() {
        let has_infix_operator = self
          .infix_parser_functions
          .contains_key(&self.peek_token.token_type);

        if !has_infix_operator {
          return expression;
        }

        self.next_token();

        let infix_parser_function = self
          .infix_parser_functions
          .get(&self.current_token.token_type)
          .unwrap();

        expression = infix_parser_function(self, expression.unwrap());
      }

      return expression;
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

  pub fn current_precedence(&mut self) -> Precedence {
    get_operator_precedence(self.current_token.token_type)
  }

  pub fn peek_precedence(&mut self) -> Precedence {
    get_operator_precedence(self.peek_token.token_type)
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
