use crate::token::*;
use crate::lexer::*;

#[test]
fn test_next_token() {
  let input = "
    let five = 5;
    let ten = 10;

    let add = fn(x, y) {
      x + y;
    };

    let result = add(five, ten);
  ";
  use token_types::*;

  let tests = vec![
    // let five = 5;
    Token { token_type: LET, literal: Some("let".to_string()) },
    Token { token_type: IDENT, literal: Some("five".to_string()) },
    Token { token_type: ASSIGN, literal: Some("=".to_string()) },
    Token { token_type: INT, literal: Some("5".to_string()) },
    Token { token_type: SEMICOLON, literal: Some(";".to_string()) },

    // let ten = 10;
    Token { token_type: LET, literal: Some("let".to_string()) },
    Token { token_type: IDENT, literal: Some("ten".to_string()) },
    Token { token_type: ASSIGN, literal: Some("=".to_string()) },
    Token { token_type: INT, literal: Some("10".to_string()) },
    Token { token_type: SEMICOLON, literal: Some(";".to_string()) },

    // let add = fn(x, y) {
    Token { token_type: LET, literal: Some("let".to_string()) },
    Token { token_type: IDENT, literal: Some("add".to_string()) },
    Token { token_type: ASSIGN, literal: Some("=".to_string()) },
    Token { token_type: FUNCTION, literal: Some("fn".to_string()) },
    Token { token_type: LPAREN, literal: Some("(".to_string()) },
    Token { token_type: IDENT, literal: Some("five".to_string()) },
    Token { token_type: COMMA, literal: Some(",".to_string()) },
    Token { token_type: IDENT, literal: Some("ten".to_string()) },
    Token { token_type: RPAREN, literal: Some(")".to_string()) },
    Token { token_type: LBRACE, literal: Some("{".to_string()) },

    // x + y
    Token { token_type: IDENT, literal: Some("x".to_string()) },
    Token { token_type: PLUS, literal: Some("+".to_string()) },
    Token { token_type: IDENT, literal: Some("y".to_string()) },
    Token { token_type: SEMICOLON, literal: Some(";".to_string()) },

    // }
    Token { token_type: RBRACE, literal: Some("}".to_string()) },
    Token { token_type: SEMICOLON, literal: Some(";".to_string()) },

    // let result = add(five, ten);
    Token { token_type: LET, literal: Some("let".to_string()) },
    Token { token_type: IDENT, literal: Some("result".to_string()) },
    Token { token_type: ASSIGN, literal: Some("=".to_string()) },
    Token { token_type: IDENT, literal: Some("add".to_string()) },
    Token { token_type: LPAREN, literal: Some("(".to_string()) },
    Token { token_type: IDENT, literal: Some("five".to_string()) },
    Token { token_type: COMMA, literal: Some(",".to_string()) },
    Token { token_type: IDENT, literal: Some("ten".to_string()) },
    Token { token_type: RPAREN, literal: Some(")".to_string()) },
    Token { token_type: SEMICOLON, literal: Some(";".to_string()) },
    Token { token_type: EOF, literal: None },
  ];

  let mut lexer = Lexer::new(input);

  for test in &tests {
    let token = lexer.next_token();

    if let Some(literal) = &token.literal {
      println!("Char: {}", *literal)
    }

    assert_eq!(token.token_type, test.token_type);
    assert_eq!(token.literal, test.literal);
  }
}
