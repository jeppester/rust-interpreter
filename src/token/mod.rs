pub type TokenType<'a> = &'a str;

pub struct Token<'a> {
  pub token_type: TokenType<'a>,
  pub literal: Option<char>,
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

  // Unknown
  pub const UNKNOWN: &str = "UNKNOWN";
}
