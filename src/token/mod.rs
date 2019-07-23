pub type TokenType<'a> = &'a str;

pub struct Token<'a> {
  pub token_type: TokenType<'a>,
  pub literal: Option<String>,
}

pub fn get_token_type_for_string<'a>(string: &str) -> TokenType<'a> {
  match string {
    "fn" => token_types::FUNCTION,
    "let" => token_types::LET,
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
}

pub const WHITESPACE_CHARS: [char; 4] = [' ', '\t', '\n', '\r'];
