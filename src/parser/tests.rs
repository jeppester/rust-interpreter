use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::ast::*;

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

  let tests = vec![
    ("x"),
    ("y"),
    ("foobar"),
  ];

  for (i, test) in tests.iter().enumerate() {
    let name = test;

    let node = &program.statements[i];

    assert_eq!(node.token_literal(), &Some("let".to_string()));

    if let Node::Statement(statement) = node {
      if let Statement::LetStatement(let_statement) = statement {
        assert_eq!(let_statement.name.value, name.to_string());
        assert_eq!(let_statement.name.token.literal, Some(name.to_string()));
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
    assert_eq!(node.token_literal(), &Some("return".to_string()));
  }
}
