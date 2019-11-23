use crate::ast::*;
use crate::lexer::Lexer;
use crate::parser::Parser;

#[test]
fn test_let_statements() {
  let input = "
    let x = 5;
    let y = 10;
    let foobar = 12345;
  ";

  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = parser.parse_program();

  assert_eq!(program.statements.len(), 3);

  let tests = vec![("x"), ("y"), ("foobar")];

  for (i, test) in tests.iter().enumerate() {
    let name = test;

    let node = &program.statements[i];

    assert_eq!(node.token_literal(), "let".to_string());

    if let Node::Statement(statement) = node {
      if let Statement::LetStatement(let_statement) = statement {
        assert_eq!(let_statement.name.value, name.to_string());
        assert_eq!(let_statement.name.token.literal, name.to_string());
      }
    }
  }
}

#[test]
fn test_return_statements() {
  let input = "
    return 5;
    return 10;
    return 12345;
  ";

  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = parser.parse_program();

  assert_eq!(program.statements.len(), 3);

  for node in &program.statements {
    assert_eq!(node.token_literal(), "return".to_string());
  }
}

#[test]
fn test_identifier_expression() {
  let input = "foobar;";

  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = parser.parse_program();

  assert_eq!(program.statements.len(), 1);

  let first_node = &program.statements[0];

  if let Node::Expression(expression) = first_node {
    if let Expression::Identifier(identifier) = expression {
      assert_eq!(identifier.value, "foobar");
      assert_eq!(identifier.token.literal, "foobar".to_string());
    }
  }
}

#[test]
fn test_integer_literal_expression() {
  let input = "5;";

  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = parser.parse_program();

  assert_eq!(program.statements.len(), 1);

  let first_node = &program.statements[0];

  if let Node::Expression(expression) = first_node {
    if let Expression::IntegerLiteral(integer_literal) = expression {
      assert_eq!(integer_literal.value, 5);
      assert_eq!(integer_literal.token.literal, "5".to_string());
    }
  }
}
