#[cfg(test)]
mod tests;

use crate::token::*;

pub struct Lexer {
  chars: Vec<char>,
  position: usize,
  read_position: usize,
  ch: Option<char>,
}

impl Lexer {
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
      self.ch = Some(self.chars[self.read_position]);
    }

    self.position = self.read_position;
    self.read_position += 1;
  }

  pub fn peak_char(&mut self) -> Option<char> {
    use std::convert::TryInto;

    if self.read_position >= self.chars.len().try_into().unwrap() {
      None
    }
    else {
      Some(self.chars[self.read_position])
    }
  }

  pub fn current_char_is_letter(&mut self) -> bool {
    if None == self.ch {
      false
    }
    else {
      let ch = self.ch.unwrap();
      ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') || ch == '_'
    }
  }

  pub fn current_char_is_digit(&mut self) -> bool {
    if None == self.ch {
      false
    }
    else {
      let ch = self.ch.unwrap();

      '0' <= ch && ch <= '9'
    }
  }

  pub fn skip_whitespace(&mut self) {
    while self.ch != None && WHITESPACE_CHARS.contains(&self.ch.unwrap()) {
      self.read_char();
    }
  }

  pub fn read_identifier(&mut self) -> String {
    let position = self.position;

    while self.current_char_is_letter() {
      self.read_char();
    }

    self.chars[position..self.position].into_iter().collect()
  }

  pub fn read_digit(&mut self) -> String {
    let position = self.position;

    while self.current_char_is_digit() {
      self.read_char();
    }

    self.chars[position..self.position].into_iter().collect()
  }

  pub fn read_string(&mut self) -> String {
    let mut string_chars = vec![];

    loop {
      self.read_char();

      match self.ch {
        None => break,
        Some(ch) => {
          match ch {
            '"' => break,
            '\\' => self.read_escape(&mut string_chars),
            x => string_chars.push(x),
          }
        }
      }
    }

    string_chars.into_iter().collect()
  }

  pub fn read_escape(&mut self, string_chars: &mut Vec<char>) {
    if let Some(_) = self.peak_char() {
      self.read_char();
      let ch = self.ch.unwrap();

      match ch {
        'n' => string_chars.push('\n'),
        'r' => string_chars.push('\r'),
        x => string_chars.push(x),
      }
    }
  }

  pub fn next_token(&mut self) -> Token {
    use crate::token::*;
    use token_types::*;

    let token;

    self.skip_whitespace();

    if self.ch == None {
      token = Token { token_type: EOF, literal: "".to_string() }
    }
    else {
      let ch = self.ch.unwrap();
      let literal = ch.to_string();

      match ch {
        ';' => token = Token { token_type: SEMICOLON, literal },
        '{' => token = Token { token_type: LBRACE, literal },
        '}' => token = Token { token_type: RBRACE, literal },
        '(' => token = Token { token_type: LPAREN, literal },
        ')' => token = Token { token_type: RPAREN, literal },
        ',' => token = Token { token_type: COMMA, literal },
        '+' => token = Token { token_type: PLUS, literal },
        '-' => token = Token { token_type: MINUS, literal },
        '*' => token = Token { token_type: ASTERISK, literal },
        '/' => token = Token { token_type: SLASH, literal },
        '<' => token = Token { token_type: LT, literal },
        '>' => token = Token { token_type: GT, literal },
        '=' => {
          if self.peak_char() == Some('=') {
            token = Token { token_type: EQ, literal: "==".to_string() };
            self.read_char();
          }
          else {
            token = Token { token_type: ASSIGN, literal };
          }
        },
        '!' => {
          if self.peak_char() == Some('=') {
            token = Token { token_type: NOT_EQ, literal: "!=".to_string() };
            self.read_char();
          }
          else {
            token = Token { token_type: BANG, literal };
          }
        },
        '"' => {
          token = Token { token_type: STRING, literal: self.read_string() }
        },
        _x => {
          if self.current_char_is_letter() {
            let literal = self.read_identifier();
            let token_type = get_token_type_for_string(&literal);

            return Token { token_type, literal }
          }
          else if self.current_char_is_digit() {
            let literal = self.read_digit();

            return Token { token_type: INT, literal }
          }
          else {
            token = Token { token_type: ILLEGAL, literal }
          }
        },
      }
    }

    self.read_char();

    return token;
  }
}
