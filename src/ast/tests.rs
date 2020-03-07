use crate::ast::*;
use crate::token::*;
use identifier::Identifier;
use let_statement::LetStatement;

#[test]
fn test_to_string() {
  let program = Program {
    statements: vec![Statement::LetStatement(LetStatement {
      token: Token {
        token_type: token_types::LET,
        literal: "let".to_string(),
      },
      name: Identifier {
        token: Token {
          token_type: token_types::IDENT,
          literal: "myVar".to_string(),
        },
        value: "myVar".to_string(),
      },
    })],
  };

  assert_eq!(program.to_string(), "let myVar = [TODO: EXPRESSION];");
}
