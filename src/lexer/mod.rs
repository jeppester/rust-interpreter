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

    let mut token;

    if let Some(ch) = self.ch {
      let literal = Some(ch.to_string());

      match ch {
        '=' => token = Token { token_type: ASSIGN, literal: literal },
        ';' => token = Token { token_type: SEMICOLON, literal: literal },
        '{' => token = Token { token_type: LBRACE, literal: literal },
        '}' => token = Token { token_type: RBRACE, literal: literal },
        '(' => token = Token { token_type: LPAREN, literal: literal },
        ')' => token = Token { token_type: RPAREN, literal: literal },
        ',' => token = Token { token_type: COMMA, literal: literal },
        '+' => token = Token { token_type: PLUS, literal: literal },
        _x => token = Token { token_type: ILLEGAL, literal: literal },
      }
    }
    else {
      token = Token { token_type: EOF, literal: None }
    }

    self.read_char();

    return token;
  }
}
