pub type TokenType = &'static str;
pub type Literal = String;

#[derive(Clone)]
#[derive(Debug)]
pub struct Token {
  pub token_type: TokenType,
  pub literal: Literal,
}

pub fn get_token_type_for_string(string: &str) -> TokenType {
  match string {
    "fn" => token_types::FUNCTION,
    "let" => token_types::LET,
    "true" => token_types::TRUE,
    "false" => token_types::FALSE,
    "if" => token_types::IF,
    "else" => token_types::ELSE,
    "return" => token_types::RETURN,
    _x => token_types::IDENT,
  }
}

pub mod token_types {
  pub const ILLEGAL: &str = "ILLEGAL";
  pub const EOF: &str = "EOF";

  // Identifiers + literals
  pub const IDENT: &str = "IDENT";
  pub const INT: &str = "INT";

  // Operators
  pub const ASSIGN: &str = "=";
  pub const PLUS: &str = "+";
  pub const MINUS: &str = "-";
  pub const BANG: &str = "!";
  pub const ASTERISK: &str = "*";
  pub const SLASH: &str = "/";
  pub const LT: &str = "<";
  pub const GT: &str = ">";
  pub const EQ: &str = "==";
  pub const NOT_EQ: &str = "!=";

  // Delimiters
  pub const COMMA: &str = ",";
  pub const SEMICOLON: &str = ";";

  pub const LPAREN: &str = "(";
  pub const RPAREN: &str = ")";
  pub const LBRACE: &str = "{";
  pub const RBRACE: &str = "}";

  // Keywords
  pub const FUNCTION: &str = "FUNCTION";
  pub const LET: &str = "LET";
  pub const TRUE: &str = "TRUE";
  pub const FALSE: &str = "FALSE";
  pub const IF: &str = "IF";
  pub const ELSE: &str = "ELSE";
  pub const RETURN: &str = "RETURN";
}

pub const WHITESPACE_CHARS: [char; 4] = [' ', '\t', '\n', '\r'];
