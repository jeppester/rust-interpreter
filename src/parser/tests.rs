use crate::ast::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::parser::ParserError;

enum LiteralValue<'a> {
  Boolean(bool),
  Identifier(&'a str),
  Integer(i64),
}

#[test]
fn test_let_statements() -> Result<(), ParserError> {
  let tests = vec![
    ("let x = 5;", "x", LiteralValue::Integer(5)),
    ("let y = z;", "y", LiteralValue::Identifier("z")),
    ("let z = false", "z", LiteralValue::Boolean(false)),
  ];

  for test in &tests {
    let (input, name, value) = test;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program()?;

    let first_statement = &program.statements[0];
    let let_statement = match_or_fail!(first_statement, Statement::LetStatement(m) => m);

    assert_eq!(let_statement.name.value, name.to_string());
    assert_eq!(let_statement.name.token.literal, name.to_string());
    assert_literal(&let_statement.value, value);
  }

  Ok(())
}

#[test]
fn test_return_statements() -> Result<(), ParserError> {
  let tests = vec![
    ("return 5;", LiteralValue::Integer(5)),
    ("return 10;", LiteralValue::Integer(10)),
    ("return 12345;", LiteralValue::Integer(12345)),
  ];

  for test in &tests {
    let (input, return_value) = test;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program()?;

    for statement in &program.statements {
      let return_statement = match_or_fail!(statement, Statement::ReturnStatement(m) => m);

      assert_eq!(return_statement.token.literal, "return".to_string());
      assert_literal(&*return_statement.return_value, return_value);
    }
  }

  Ok(())
}

#[test]
fn test_identifier_expression() -> Result<(), ParserError> {
  let input = "foobar;";

  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = parser.parse_program()?;

  assert_eq!(program.statements.len(), 1);

  let first_statement = &program.statements[0];
  let expression = match_or_fail!(first_statement, Statement::Expression(m) => m);

  assert_identifier(expression, "foobar");

  Ok(())
}

#[test]
fn test_integer_literal_expression() -> Result<(), ParserError> {
  let input = "5;";

  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = parser.parse_program()?;

  assert_eq!(program.statements.len(), 1);

  let first_statement = &program.statements[0];
  let expression = match_or_fail!(first_statement, Statement::Expression(m) => m);

  assert_integer_literal(expression, &5);

  Ok(())
}

#[test]
fn test_string_literal_expression() -> Result<(), ParserError> {
  let input = "\"hello world\"";

  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = parser.parse_program()?;

  assert_eq!(program.statements.len(), 1);

  let first_statement = &program.statements[0];
  let expression = match_or_fail!(first_statement, Statement::Expression(m) => m);

  assert_string_literal(expression, "hello world");

  Ok(())
}

#[test]
fn test_boolean_expression() -> Result<(), ParserError> {
  let tests = vec![("true;", true), ("false;", false)];

  for test in &tests {
    let (input, value) = test;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program()?;

    assert_eq!(program.statements.len(), 1);

    let first_statement = &program.statements[0];
    let expression = match_or_fail!(first_statement, Statement::Expression(m) => m);

    assert_boolean(expression, value);
  }

  Ok(())
}

#[test]
fn test_prefix_expressions() -> Result<(), ParserError> {
  let tests = vec![
    ("!5", "!", LiteralValue::Integer(5)),
    ("-15", "-", LiteralValue::Integer(15)),
    ("!true", "!", LiteralValue::Boolean(true)),
    ("!false", "!", LiteralValue::Boolean(false)),
  ];

  for test in &tests {
    let (input, operator, right) = test;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program()?;

    assert_eq!(program.statements.len(), 1);

    let first_statement = &program.statements[0];
    let expression = match_or_fail!(first_statement, Statement::Expression(m) => m);

    assert_prefix(expression, operator.to_string(), right);
  }

  Ok(())
}

#[test]
fn test_infix_expressions() -> Result<(), ParserError> {
  let tests = vec![
    (
      "5 + 5",
      LiteralValue::Integer(5),
      "+",
      LiteralValue::Integer(5),
    ),
    (
      "5 - 5",
      LiteralValue::Integer(5),
      "-",
      LiteralValue::Integer(5),
    ),
    (
      "5 * 5",
      LiteralValue::Integer(5),
      "*",
      LiteralValue::Integer(5),
    ),
    (
      "5 / 5",
      LiteralValue::Integer(5),
      "/",
      LiteralValue::Integer(5),
    ),
    (
      "5 < 5",
      LiteralValue::Integer(5),
      "<",
      LiteralValue::Integer(5),
    ),
    (
      "5 > 5",
      LiteralValue::Integer(5),
      ">",
      LiteralValue::Integer(5),
    ),
    (
      "5 == 5",
      LiteralValue::Integer(5),
      "==",
      LiteralValue::Integer(5),
    ),
    (
      "5 != 5",
      LiteralValue::Integer(5),
      "!=",
      LiteralValue::Integer(5),
    ),
    (
      "true == true",
      LiteralValue::Boolean(true),
      "==",
      LiteralValue::Boolean(true),
    ),
    (
      "true != false",
      LiteralValue::Boolean(true),
      "!=",
      LiteralValue::Boolean(false),
    ),
    (
      "false == false",
      LiteralValue::Boolean(false),
      "==",
      LiteralValue::Boolean(false),
    ),
  ];

  for test in &tests {
    let (input, left_value, operator, right_value) = test;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program()?;

    assert_eq!(program.statements.len(), 1);

    let first_statement = &program.statements[0];
    let expression = match_or_fail!(first_statement, Statement::Expression(m) => m);

    assert_infix(expression, left_value, operator, right_value);
  }

  Ok(())
}

#[test]
fn test_operator_precedence_parsing() -> Result<(), ParserError> {
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
    ("true", "true"),
    ("false", "false"),
    ("3 > 5 == false", "((3 > 5) == false)"),
    ("3 < 5 == true", "((3 < 5) == true)"),
    ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
    ("(5 + 5) * 2", "((5 + 5) * 2)"),
    ("2 / (5 + 5)", "(2 / (5 + 5))"),
    ("-(5 + 5)", "(-(5 + 5))"),
    ("!(true == true)", "(!(true == true))"),
    ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
    (
      "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
      "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
    ),
    (
      "add(a + b + c * d / f + g)",
      "add((((a + b) + ((c * d) / f)) + g))",
    ),
  ];

  for test in &tests {
    let (input, expected) = test;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program()?;

    assert_eq!(&program.to_string(), expected);
  }

  Ok(())
}

#[test]
fn test_if_expression() -> Result<(), ParserError> {
  let input = "if (x < y) { x }";

  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = parser.parse_program()?;

  assert_eq!(program.statements.len(), 1);

  let first_statement = &program.statements[0];
  let if_expression = match_or_fail!(
    first_statement,
    Statement::Expression(Expression::IfExpression(m)) => m
  );

  assert_infix(
    &if_expression.condition,
    &LiteralValue::Identifier("x"),
    "<",
    &LiteralValue::Identifier("y"),
  );

  assert_eq!(if_expression.true_block.statements.len(), 1);
  let first_true_block_statement = &if_expression.true_block.statements[0];
  let first_true_block_expression = match_or_fail!(first_true_block_statement, Statement::Expression(m) => m);
  assert_literal(first_true_block_expression, &LiteralValue::Identifier("x"));

  match_or_fail!(&*if_expression.false_block_or_none, None => ());

  Ok(())
}

#[test]
fn test_if_else_expression() -> Result<(), ParserError> {
  let input = "if (x < y) { x } else { y }";

  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = parser.parse_program()?;

  assert_eq!(program.statements.len(), 1);

  let first_statement = &program.statements[0];
  let if_expression = match_or_fail!(
    first_statement,
    Statement::Expression(Expression::IfExpression(m)) => m
  );

  assert_infix(
    &if_expression.condition,
    &LiteralValue::Identifier("x"),
    "<",
    &LiteralValue::Identifier("y"),
  );

  assert_eq!(if_expression.true_block.statements.len(), 1);
  let first_true_block_statement = &if_expression.true_block.statements[0];
  let first_true_block_expression = match_or_fail!(first_true_block_statement, Statement::Expression(m) => m);
  assert_literal(first_true_block_expression, &LiteralValue::Identifier("x"));

  let false_block = match_or_fail!(&*if_expression.false_block_or_none, Some(m) => m);
  let first_false_block_statement = &false_block.statements[0];
  let first_false_block_expression = match_or_fail!(first_false_block_statement, Statement::Expression(m) => m);
  assert_literal(first_false_block_expression, &LiteralValue::Identifier("y"));

  Ok(())
}

#[test]
fn test_function_literal() -> Result<(), ParserError> {
  let input = "fn (x, y) { x + y };";

  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = parser.parse_program()?;

  assert_eq!(program.statements.len(), 1);

  let first_statement = &program.statements[0];
  let function_literal = match_or_fail!(
    first_statement,
    Statement::Expression(Expression::FunctionLiteral(m)) => m
  );

  assert_eq!(function_literal.token.literal, "fn");

  assert_eq!(function_literal.arguments.len(), 2);
  let first_argument = &function_literal.arguments[0];
  assert_eq!(first_argument.value, "x");
  assert_eq!(first_argument.token.literal, "x".to_string());

  let second_argument = &function_literal.arguments[1];
  assert_eq!(second_argument.value, "y");
  assert_eq!(second_argument.token.literal, "y".to_string());

  assert_eq!(function_literal.body.statements.len(), 1);
  let first_body_statement = &function_literal.body.statements[0];
  let first_body_expression = match_or_fail!(first_body_statement, Statement::Expression(m) => m);
  assert_infix(
    &first_body_expression,
    &LiteralValue::Identifier("x"),
    "+",
    &LiteralValue::Identifier("y"),
  );

  Ok(())
}

#[test]
fn test_function_parameter_parsing() -> Result<(), ParserError> {
  let tests = vec![
    ("fn() {}", vec![]),
    ("fn(x) {}", vec!["x"]),
    ("fn(x, y, z) {}", vec!["x", "y", "z"]),
  ];

  for test in &tests {
    let (input, identifiers) = test;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program()?;
    assert_eq!(program.statements.len(), 1);

    let first_statement = &program.statements[0];
    let function_literal = match_or_fail!(
      first_statement,
      Statement::Expression(Expression::FunctionLiteral(m)) => m
    );

    assert_eq!(function_literal.arguments.len(), identifiers.len());

    for (i, identifier) in identifiers.iter().enumerate() {
      let argument = &function_literal.arguments[i];

      assert_eq!(argument.value, *identifier);
      assert_eq!(argument.token.literal, identifier.to_string());
    }
  }

  Ok(())
}

#[test]
fn test_call_expression() -> Result<(), ParserError> {
  let input = "add(1, 2 * 3, 4 + 5);";

  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = parser.parse_program()?;

  assert_eq!(program.statements.len(), 1);

  let first_statement = &program.statements[0];
  let call_expression = match_or_fail!(
    first_statement,
    Statement::Expression(Expression::CallExpression(m)) => m
  );

  assert_identifier(&*call_expression.function, "add");
  assert_eq!(call_expression.arguments.len(), 3);
  assert_integer_literal(&call_expression.arguments[0], &1);
  assert_infix(
    &call_expression.arguments[1],
    &LiteralValue::Integer(2),
    "*",
    &LiteralValue::Integer(3),
  );
  assert_infix(
    &call_expression.arguments[2],
    &LiteralValue::Integer(4),
    "+",
    &LiteralValue::Integer(5),
  );

  Ok(())
}

fn assert_boolean(expression: &Expression, value: &bool) {
  if let Expression::BooleanLiteral(boolean_literal) = expression {
    assert_eq!(&boolean_literal.value, value);
    assert_eq!(
      &boolean_literal.token.literal,
      if *value { "true" } else { "false" }
    );
  } else {
    panic!("Expected boolean literal expression, got {:?}", expression);
  }
}

fn assert_identifier(expression: &Expression, value: &str) {
  if let Expression::Identifier(identifier) = expression {
    assert_eq!(identifier.value, value);
    assert_eq!(identifier.token.literal, value.to_string());
  } else {
    panic!("Expected identifier expression, got {:?}", expression);
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

fn assert_string_literal(expression: &Expression, value: &str) {
  if let Expression::StringLiteral(string_literal) = expression {
    assert_eq!(&string_literal.value, value);
    assert_eq!(string_literal.token.literal, value);
  } else {
    panic!("Expected string literal expression, got {:?}", expression)
  }
}

fn assert_literal(expression: &Expression, value: &LiteralValue) {
  match value {
    LiteralValue::Boolean(boolean_value) => assert_boolean(expression, &boolean_value),
    LiteralValue::Identifier(identifier_value) => assert_identifier(expression, &identifier_value),
    LiteralValue::Integer(integer_value) => assert_integer_literal(expression, &integer_value),
  }
}

fn assert_prefix(expression: &Expression, operator: String, right: &LiteralValue) {
  if let Expression::PrefixExpression(prefix_expression) = expression {
    assert_eq!(prefix_expression.operator, operator);
    assert_literal(&*prefix_expression.right, right);
  } else {
    panic!("Expected prefix expression, got {:?}", expression)
  }
}

fn assert_infix(
  expression: &Expression,
  left: &LiteralValue,
  operator: &str,
  right: &LiteralValue,
) {
  if let Expression::InfixExpression(infix_expression) = expression {
    assert_literal(&*infix_expression.left, left);
    assert_eq!(infix_expression.operator, operator);
    assert_literal(&*infix_expression.right, right);
  } else {
    panic!("Expected infix expression, got {:?}", expression)
  }
}
