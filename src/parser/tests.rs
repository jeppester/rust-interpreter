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

    let first_node = &program.statements[i];

    assert_eq!(first_node.token_literal(), "let".to_string());

    if let Node::Statement(Statement::LetStatement(let_statement)) = first_node {
      assert_eq!(let_statement.name.value, name.to_string());
      assert_eq!(let_statement.name.token.literal, name.to_string());
    } else {
      panic!("Expected let statement, got {:?}", first_node)
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
    if let Node::Statement(Statement::ReturnStatement(return_statement)) = node {
      assert_eq!(return_statement.token.literal, "return".to_string());
    } else {
      panic!("Expected return statement, got {:?}", node)
    }
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

  if let Node::Expression(Expression::Identifier(identifier)) = first_node {
    assert_eq!(identifier.value, "foobar");
    assert_eq!(identifier.token.literal, "foobar".to_string());
  } else {
    panic!("Expected identifier expression, got {:?}", first_node)
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
    assert_integer_literal(expression, &5)
  } else {
    panic!("Expected integer literal expression, got {:?}", first_node)
  }
}

#[test]
fn test_boolean_expression() {
  let tests = vec![("true;", "true", true), ("false;", "false", false)];

  for test in &tests {
    let (input, literal, value) = test;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    assert_eq!(program.statements.len(), 1);

    let first_node = &program.statements[0];

    if let Node::Expression(Expression::Boolean(boolean)) = first_node {
      assert_eq!(&boolean.value, value);
      assert_eq!(boolean.token.literal, literal.to_string());
    } else {
      panic!("Expected boolean expression, got {:?}", first_node)
    }
  }
}

#[test]
fn test_prefix_expressions() {
  let tests = vec![("!5", "!", 5), ("-15", "-", 15)];

  for test in &tests {
    let (input, operator, integer_value) = test;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    assert_eq!(program.statements.len(), 1);

    let first_node = &program.statements[0];

    if let Node::Expression(Expression::PrefixExpression(prefix_expression)) = first_node {
      assert_eq!(prefix_expression.operator, operator.to_string());
      assert_integer_literal(&*prefix_expression.right, integer_value);
    } else {
      panic!("Expected prefix expression, got {:?}", first_node)
    }
  }
}

#[test]
fn test_infix_expressions() {
  let tests = vec![
    ("5 + 5", 5, "+", 5),
    ("5 - 5", 5, "-", 5),
    ("5 * 5", 5, "*", 5),
    ("5 / 5", 5, "/", 5),
    ("5 < 5", 5, "<", 5),
    ("5 > 5", 5, ">", 5),
    ("5 == 5", 5, "==", 5),
    ("5 != 5", 5, "!=", 5),
  ];

  for test in &tests {
    let (input, left_value, operator, right_value) = test;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    assert_eq!(program.statements.len(), 1);

    let first_node = &program.statements[0];

    if let Node::Expression(Expression::InfixExpression(infix_expression)) = first_node {
      assert_eq!(infix_expression.operator, operator.to_string());
      assert_integer_literal(&*infix_expression.left, left_value);
      assert_integer_literal(&*infix_expression.right, right_value);
    } else {
      panic!("Expected infix expression, got {:?}", first_node)
    }
  }
}

#[test]
fn test_operator_precedence_parsing() {
  let tests = vec![
    ("-a * b", "((-a) * b)"),
    ("!-a", "(!(-a))"),
    ("a + b + c", "((a + b) + c)"),
    ("a + b - c", "((a + b) - c)"),
    ("a * b * c", "((a * b) * c)"),
    ("a * b / c", "((a * b) / c)"),
    ("a + b / c", "(a + (b / c))"),
    ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
    ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
    ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
    ("5 < 4 != 3 < 4", "((5 < 4) != (3 < 4))"),
    (
      "3 + 4 * 5 == 3 * 1 + 4 * 5",
      "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
    ),
  ];

  for test in &tests {
    let (input, expected) = test;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    assert_eq!(&program.to_string(), expected);
  }
}

fn assert_integer_literal(expression: &Expression, value: &i64) {
  if let Expression::IntegerLiteral(integer_literal) = expression {
    assert_eq!(&integer_literal.value, value);
    assert_eq!(integer_literal.token.literal, value.to_string());
  } else {
    panic!("Expected integer literal expression, got {:?}", expression)
  }
}
