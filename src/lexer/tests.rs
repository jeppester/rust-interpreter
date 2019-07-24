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
    !-/*5;
    5 < 10 > 5;

    if (5 < 10) {
      return true;
    }
    else {
      return false;
    }

    10 == 10;
    10 != 9;
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
    Token { token_type: IDENT, literal: Some("x".to_string()) },
    Token { token_type: COMMA, literal: Some(",".to_string()) },
    Token { token_type: IDENT, literal: Some("y".to_string()) },
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

    // !-/*5;
    Token { token_type: BANG, literal: Some("!".to_string()) },
    Token { token_type: MINUS, literal: Some("-".to_string()) },
    Token { token_type: SLASH, literal: Some("/".to_string()) },
    Token { token_type: ASTERISK, literal: Some("*".to_string()) },
    Token { token_type: INT, literal: Some("5".to_string()) },
    Token { token_type: SEMICOLON, literal: Some(";".to_string()) },

    // 5 < 10 > 5;
    Token { token_type: INT, literal: Some("5".to_string()) },
    Token { token_type: LT, literal: Some("<".to_string()) },
    Token { token_type: INT, literal: Some("10".to_string()) },
    Token { token_type: GT, literal: Some(">".to_string()) },
    Token { token_type: INT, literal: Some("5".to_string()) },
    Token { token_type: SEMICOLON, literal: Some(";".to_string()) },

    // if (5 < 10) {
    Token { token_type: IF, literal: Some("if".to_string()) },
    Token { token_type: LPAREN, literal: Some("(".to_string()) },
    Token { token_type: INT, literal: Some("5".to_string()) },
    Token { token_type: LT, literal: Some("<".to_string()) },
    Token { token_type: INT, literal: Some("10".to_string()) },
    Token { token_type: RPAREN, literal: Some(")".to_string()) },
    Token { token_type: LBRACE, literal: Some("{".to_string()) },

    // return true;
    Token { token_type: RETURN, literal: Some("return".to_string()) },
    Token { token_type: TRUE, literal: Some("true".to_string()) },
    Token { token_type: SEMICOLON, literal: Some(";".to_string()) },

    // }
    Token { token_type: RBRACE, literal: Some("}".to_string()) },

    // else {
    Token { token_type: ELSE, literal: Some("else".to_string()) },
    Token { token_type: LBRACE, literal: Some("{".to_string()) },

    //   return false;
    Token { token_type: RETURN, literal: Some("return".to_string()) },
    Token { token_type: FALSE, literal: Some("false".to_string()) },
    Token { token_type: SEMICOLON, literal: Some(";".to_string()) },

    // }
    Token { token_type: RBRACE, literal: Some("}".to_string()) },

    // 10 == 10;
    Token { token_type: INT, literal: Some("10".to_string()) },
    Token { token_type: EQ, literal: Some("==".to_string()) },
    Token { token_type: INT, literal: Some("10".to_string()) },
    Token { token_type: SEMICOLON, literal: Some(";".to_string()) },

    // 10 != 9;
    Token { token_type: INT, literal: Some("10".to_string()) },
    Token { token_type: NOT_EQ, literal: Some("!=".to_string()) },
    Token { token_type: INT, literal: Some("9".to_string()) },
    Token { token_type: SEMICOLON, literal: Some(";".to_string()) },

    // END
    Token { token_type: EOF, literal: None },
  ];

  let mut lexer = Lexer::new(input);

  for test in &tests {
    let token = lexer.next_token();

    if let Some(literal) = &token.literal {
      println!("Type: {}, Char: {}", token.token_type, *literal)
    }

    assert_eq!(token.token_type, test.token_type);
    assert_eq!(token.literal, test.literal);
  }
}
