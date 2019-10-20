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
    Token { token_type: LET, literal: "let".to_string() },
    Token { token_type: IDENT, literal: "five".to_string() },
    Token { token_type: ASSIGN, literal: "=".to_string() },
    Token { token_type: INT, literal: "5".to_string() },
    Token { token_type: SEMICOLON, literal: ";".to_string() },

    // let ten = 10;
    Token { token_type: LET, literal: "let".to_string() },
    Token { token_type: IDENT, literal: "ten".to_string() },
    Token { token_type: ASSIGN, literal: "=".to_string() },
    Token { token_type: INT, literal: "10".to_string() },
    Token { token_type: SEMICOLON, literal: ";".to_string() },

    // let add = fn(x, y) {
    Token { token_type: LET, literal: "let".to_string() },
    Token { token_type: IDENT, literal: "add".to_string() },
    Token { token_type: ASSIGN, literal: "=".to_string() },
    Token { token_type: FUNCTION, literal: "fn".to_string() },
    Token { token_type: LPAREN, literal: "(".to_string() },
    Token { token_type: IDENT, literal: "x".to_string() },
    Token { token_type: COMMA, literal: ",".to_string() },
    Token { token_type: IDENT, literal: "y".to_string() },
    Token { token_type: RPAREN, literal: ")".to_string() },
    Token { token_type: LBRACE, literal: "{".to_string() },

    // x + y
    Token { token_type: IDENT, literal: "x".to_string() },
    Token { token_type: PLUS, literal: "+".to_string() },
    Token { token_type: IDENT, literal: "y".to_string() },
    Token { token_type: SEMICOLON, literal: ";".to_string() },

    // }
    Token { token_type: RBRACE, literal: "}".to_string() },
    Token { token_type: SEMICOLON, literal: ";".to_string() },

    // let result = add(five, ten);
    Token { token_type: LET, literal: "let".to_string() },
    Token { token_type: IDENT, literal: "result".to_string() },
    Token { token_type: ASSIGN, literal: "=".to_string() },
    Token { token_type: IDENT, literal: "add".to_string() },
    Token { token_type: LPAREN, literal: "(".to_string() },
    Token { token_type: IDENT, literal: "five".to_string() },
    Token { token_type: COMMA, literal: ",".to_string() },
    Token { token_type: IDENT, literal: "ten".to_string() },
    Token { token_type: RPAREN, literal: ")".to_string() },
    Token { token_type: SEMICOLON, literal: ";".to_string() },

    // !-/*5;
    Token { token_type: BANG, literal: "!".to_string() },
    Token { token_type: MINUS, literal: "-".to_string() },
    Token { token_type: SLASH, literal: "/".to_string() },
    Token { token_type: ASTERISK, literal: "*".to_string() },
    Token { token_type: INT, literal: "5".to_string() },
    Token { token_type: SEMICOLON, literal: ";".to_string() },

    // 5 < 10 > 5;
    Token { token_type: INT, literal: "5".to_string() },
    Token { token_type: LT, literal: "<".to_string() },
    Token { token_type: INT, literal: "10".to_string() },
    Token { token_type: GT, literal: ">".to_string() },
    Token { token_type: INT, literal: "5".to_string() },
    Token { token_type: SEMICOLON, literal: ";".to_string() },

    // if (5 < 10) {
    Token { token_type: IF, literal: "if".to_string() },
    Token { token_type: LPAREN, literal: "(".to_string() },
    Token { token_type: INT, literal: "5".to_string() },
    Token { token_type: LT, literal: "<".to_string() },
    Token { token_type: INT, literal: "10".to_string() },
    Token { token_type: RPAREN, literal: ")".to_string() },
    Token { token_type: LBRACE, literal: "{".to_string() },

    // return true;
    Token { token_type: RETURN, literal: "return".to_string() },
    Token { token_type: TRUE, literal: "true".to_string() },
    Token { token_type: SEMICOLON, literal: ";".to_string() },

    // }
    Token { token_type: RBRACE, literal: "}".to_string() },

    // else {
    Token { token_type: ELSE, literal: "else".to_string() },
    Token { token_type: LBRACE, literal: "{".to_string() },

    //   return false;
    Token { token_type: RETURN, literal: "return".to_string() },
    Token { token_type: FALSE, literal: "false".to_string() },
    Token { token_type: SEMICOLON, literal: ";".to_string() },

    // }
    Token { token_type: RBRACE, literal: "}".to_string() },

    // 10 == 10;
    Token { token_type: INT, literal: "10".to_string() },
    Token { token_type: EQ, literal: "==".to_string() },
    Token { token_type: INT, literal: "10".to_string() },
    Token { token_type: SEMICOLON, literal: ";".to_string() },

    // 10 != 9;
    Token { token_type: INT, literal: "10".to_string() },
    Token { token_type: NOT_EQ, literal: "!=".to_string() },
    Token { token_type: INT, literal: "9".to_string() },
    Token { token_type: SEMICOLON, literal: ";".to_string() },

    // END
    Token { token_type: EOF, literal: "".to_string() },
  ];

  let mut lexer = Lexer::new(input);

  for test in &tests {
    let token = lexer.next_token();

    println!("Type: {}, Char: {}", token.token_type, &token.literal);

    assert_eq!(token.token_type, test.token_type);
    assert_eq!(token.literal, test.literal);
  }
}
