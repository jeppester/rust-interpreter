#[cfg(test)]
mod tests;

use crate::token::*;

struct Lexer {
  chars: Vec<char>,
  position: usize,
  read_position: usize,
  ch: Option<char>,
}

impl<'a> Lexer {
  pub fn new(input: &str) -> Lexer {
    let mut lexer = Lexer {
      chars: input.chars().collect(),
      position: 0,
      read_position: 0,
      ch: None,
    };

    lexer.read_char();

    return lexer;
  }

  pub fn read_char(&mut self) {
    use std::convert::TryInto;

    if self.read_position >= self.chars.len().try_into().unwrap() {
      self.ch = None;
    }
    else {
      println!("Read position: {}", self.read_position);
      self.ch = Some(self.chars[self.read_position]);
    }

    self.position = self.read_position;
    self.read_position += 1;
  }

  pub fn next_token(&mut self) -> Token {
    use crate::token::*;
    use token_types::*;

    let token;
    match self.ch {
      Some('=') => token = Token { token_type: ASSIGN, literal: self.ch },
      Some(';') => token = Token { token_type: SEMICOLON, literal: self.ch },
      Some('{') => token = Token { token_type: LBRACE, literal: self.ch },
      Some('}') => token = Token { token_type: RBRACE, literal: self.ch },
      Some('(') => token = Token { token_type: LPAREN, literal: self.ch },
      Some(')') => token = Token { token_type: RPAREN, literal: self.ch },
      Some(',') => token = Token { token_type: COMMA, literal: self.ch },
      Some('+') => token = Token { token_type: PLUS, literal: self.ch },
      Some(_x) => token = Token { token_type: UNKNOWN, literal: self.ch },
      None => token = Token { token_type: EOF, literal: self.ch },
    }

    self.read_char();

    return token;
  }
}
