use crate::ast::*;
use crate::lexer::Lexer;
use crate::parser::Parser;

enum LiteralValue<'a> {
  Boolean(bool),
  Identifier(&'a str),
  Integer(i64),
}

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

    let first_statement = &program.statements[i];

    assert_eq!(first_statement.token_literal(), "let".to_string());

    if let Statement::LetStatement(let_statement) = first_statement {
      assert_eq!(let_statement.name.value, name.to_string());
      assert_eq!(let_statement.name.token.literal, name.to_string());
    } else {
      panic!("Expected let statement, got {:?}", first_statement)
    }
  }
}

#[test]
fn test_return_statements() {
  let tests = vec![
    ("return 5;", LiteralValue::Integer(5)),
    ("return 10;", LiteralValue::Integer(10)),
    ("return 12345;", LiteralValue::Integer(12345)),
  ];

  for test in &tests {
    let (input, return_value) = test;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    for statement in &program.statements {
      if let Statement::ReturnStatement(return_statement) = statement {
        assert_eq!(return_statement.token.literal, "return".to_string());
        assert_literal(&*return_statement.return_value, return_value)
      } else {
        panic!("Expected return statement, got {:?}", statement)
      }
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

  let first_statement = &program.statements[0];

  if let Statement::Expression(expression) = first_statement {
    assert_identifier(expression, "foobar")
  } else {
    panic!("Expected expression statement, got {:?}", first_statement)
  }
}

#[test]
fn test_integer_literal_expression() {
  let input = "5;";

  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = parser.parse_program();

  assert_eq!(program.statements.len(), 1);

  let first_statement = &program.statements[0];

  if let Statement::Expression(expression) = first_statement {
    assert_integer_literal(expression, &5)
  } else {
    panic!("Expected expression statement, got {:?}", first_statement)
  }
}

#[test]
fn test_boolean_expression() {
  let tests = vec![("true;", true), ("false;", false)];

  for test in &tests {
    let (input, value) = test;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    assert_eq!(program.statements.len(), 1);

    let first_statement = &program.statements[0];

    if let Statement::Expression(expression) = first_statement {
      assert_boolean(expression, value)
    } else {
      panic!("Expected expression statement, got {:?}", first_statement)
    }
  }
}

#[test]
fn test_prefix_expressions() {
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

    let program = parser.parse_program();

    assert_eq!(program.statements.len(), 1);

    let first_statement = &program.statements[0];

    if let Statement::Expression(expression) = first_statement {
      assert_prefix(expression, operator.to_string(), right)
    } else {
      panic!("Expected expression statement, got {:?}", first_statement)
    }
  }
}

#[test]
fn test_infix_expressions() {
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

    let program = parser.parse_program();

    assert_eq!(program.statements.len(), 1);

    let first_statement = &program.statements[0];

    if let Statement::Expression(expression) = first_statement {
      assert_infix(expression, left_value, operator.to_string(), right_value)
    } else {
      panic!("Expected expression statement, got {:?}", first_statement)
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
    ("true", "true"),
    ("false", "false"),
    ("3 > 5 == false", "((3 > 5) == false)"),
    ("3 < 5 == true", "((3 < 5) == true)"),
    ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
    ("(5 + 5) * 2", "((5 + 5) * 2)"),
    ("2 / (5 + 5)", "(2 / (5 + 5))"),
    ("-(5 + 5)", "(-(5 + 5))"),
    ("!(true == true)", "(!(true == true))"),
  ];

  for test in &tests {
    let (input, expected) = test;

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    assert_eq!(&program.to_string(), expected);
  }
}

#[test]
fn test_if_expression() {
  let input = "if (x < y) { x }";

  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = parser.parse_program();

  assert_eq!(program.statements.len(), 1);

  let first_statement = &program.statements[0];
  if let Statement::Expression(Expression::IfExpression(if_expression)) = first_statement {
    assert_infix(
      &if_expression.condition,
      &LiteralValue::Identifier("x"),
      "<".to_string(),
      &LiteralValue::Identifier("y"),
    );

    assert_eq!(if_expression.true_block.statements.len(), 1);
    let first_true_block_statement = &if_expression.true_block.statements[0];

    if let Statement::Expression(expression) = first_true_block_statement {
      assert_literal(expression, &LiteralValue::Identifier("x"));
    } else {
      panic!("Expected expression statement, got {:?}", first_statement);
    }

    if let Some(statement) = &*if_expression.false_block_or_none {
      panic!("Expected none, got {:?}", statement);
    }
  } else {
    panic!("Expected expression statement, got {:?}", first_statement);
  }
}

#[test]
fn test_if_else_expression() {
  let input = "if (x < y) { x } else { y }";

  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = parser.parse_program();

  assert_eq!(program.statements.len(), 1);

  let first_statement = &program.statements[0];
  if let Statement::Expression(Expression::IfExpression(if_expression)) = first_statement {
    assert_infix(
      &if_expression.condition,
      &LiteralValue::Identifier("x"),
      "<".to_string(),
      &LiteralValue::Identifier("y"),
    );

    assert_eq!(if_expression.true_block.statements.len(), 1);
    let first_true_block_statement = &if_expression.true_block.statements[0];

    if let Statement::Expression(expression) = first_true_block_statement {
      assert_literal(expression, &LiteralValue::Identifier("x"));
    } else {
      panic!("Expected expression statement, got {:?}", first_statement);
    }

    if let Some(false_block) = &*if_expression.false_block_or_none {
      let first_false_block_statement = &false_block.statements[0];

      if let Statement::Expression(expression) = first_false_block_statement {
        assert_literal(expression, &LiteralValue::Identifier("y"));
      } else {
        panic!("Expected expression statement, got {:?}", first_statement);
      }
    } else {
      panic!("Expected Some(BlockStatement), got None");
    }
  } else {
    panic!("Expected expression statement, got {:?}", first_statement);
  }
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
  operator: String,
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
