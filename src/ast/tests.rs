use crate::ast::*;
use crate::token::*;
use block_statement::BlockStatement;
use function_literal::FunctionLiteral;
use identifier::Identifier;
use integer_literal::IntegerLiteral;
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
      value: Expression::FunctionLiteral(FunctionLiteral {
        token: Token {
          token_type: token_types::FUNCTION,
          literal: "fn".to_string(),
        },
        arguments: vec![Identifier {
          token: Token {
            token_type: token_types::IDENT,
            literal: "param".to_string(),
          },
          value: "param".to_string(),
        }],
        body: Box::new(BlockStatement {
          token: Token {
            token_type: token_types::LBRACE,
            literal: "{".to_string(),
          },
          statements: vec![Statement::ReturnStatement(ReturnStatement {
            token: Token {
              token_type: token_types::RETURN,
              literal: "return".to_string(),
            },
            return_value: Box::new(Expression::IntegerLiteral(IntegerLiteral {
              token: Token {
                token_type: token_types::INT,
                literal: "10".to_string(),
              },
              value: 10,
            })),
          })],
        }),
      }),
    })],
  };

  assert_eq!(
    program.to_string(),
    "let myVar = fn (param) {\nreturn 10;\n}\n;"
  );
}
