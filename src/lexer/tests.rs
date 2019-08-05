use crate::token::*;
use crate::lexer::*;

#[test]
fn test_next_token() {
  let input = "=+(){},;";
  use token_types::*;

  let tests = vec![
    Token { token_type: ASSIGN, literal: Some('=') },
    Token { token_type: PLUS, literal: Some('+') },
    Token { token_type: LPAREN, literal: Some('(') },
    Token { token_type: RPAREN, literal: Some(')') },
    Token { token_type: LBRACE, literal: Some('{') },
    Token { token_type: RBRACE, literal: Some('}') },
    Token { token_type: COMMA, literal: Some(',') },
    Token { token_type: SEMICOLON, literal: Some(';') },
    Token { token_type: EOF, literal: None },
  ];

  let mut lexer = Lexer::new(input);

  for test in &tests {
    let token = lexer.next_token();

    if let Some(literal) = token.literal {
      println!("Char: {}", literal)
    }

    // assert_eq!(token.token_type, test.token_type);
    // assert_eq!(token.literal, test.literal);
  }
}
