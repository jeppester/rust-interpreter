use crate::eval::*;
use crate::lexer::*;
use crate::parser::*;
use crate::object::*;

#[test]
fn test_eval_integer_expression() -> Result<(), String> {
  let tests = vec![
    ("5", 5),
    ("10", 10),
    ("-5", -5),
    ("-10", -10),
  ];

  for test in &tests {
    let (input, result) = test;
    let result_object = test_eval(input);
    test_integer_object(&result_object, result);
  }

  Ok(())
}

#[test]
fn test_eval_boolean_expression() -> Result<(), String> {
  let tests = vec![
    ("true", true),
    ("false", false),
  ];

  for test in &tests {
    let (input, result) = test;
    let result_object = test_eval(input);
    test_boolean_object(&result_object, result);
  }

  Ok(())
}

#[test]
fn test_eval_bang_operator() -> Result<(), String> {
  let tests = vec![
    ("!true", false),
    ("!false", true),
    ("!5", false),
    ("!0", true),
    ("!!true", true),
    ("!!false", false),
    ("!!5", true),
    ("!!0", false),
  ];

  for test in &tests {
    let (input, result) = test;
    let result_object = test_eval(input);
    println!("input: {}, result: {}", input, result);
    test_boolean_object(&result_object, result);
  }

  Ok(())
}

fn test_eval(input: &str) -> Box<dyn Object> {
  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = match_or_fail!(parser.parse_program(), Ok(m) => m);
  match_or_fail!(eval(&program), Ok(m) => m)
}

fn test_integer_object(object: &Box<dyn Object>, result: &i64) {
  match_or_fail!(object.get_type(), ObjectType::Integer => ());
  let numeric_value = match_or_fail!(object.get_numeric_value(), Ok(m) => m);
  assert_eq!(numeric_value, result);
}

fn test_boolean_object(object: &Box<dyn Object>, result: &bool) {
  match_or_fail!(object.get_type(), ObjectType::Boolean => ());
  let boolean_value = match_or_fail!(object.get_boolean_value(), Ok(m) => m);
  assert_eq!(boolean_value, result);
}
