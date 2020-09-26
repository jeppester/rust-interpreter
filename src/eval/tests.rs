use crate::eval::*;
use crate::lexer::*;
use crate::parser::*;
use crate::object::*;
use crate::object::environment::*;
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_eval_integer_expression() -> Result<(), String> {
  let tests = vec![
    ("5", Object::Integer(5)),
    ("10", Object::Integer(10)),
    ("-5", Object::Integer(-5)),
    ("5 + 5 + 5 + 5 - 10", Object::Integer(10)),
    ("2 * 2 * 2 * 2 * 2", Object::Integer(32)),
    ("-50 + 100 + -50", Object::Integer(0)),
    ("5 * 2 + 10", Object::Integer(20)),
    ("5 + 2 * 10", Object::Integer(25)),
    ("20 + 2 * -10", Object::Integer(0)),
    ("50 / 2 * 2 + 10", Object::Integer(60)),
    ("2 * (5 + 10)", Object::Integer(30)),
    ("3 * 3 * 3 + 10", Object::Integer(37)),
    ("3 * (3 * 3) + 10", Object::Integer(37)),
    ("(5 + 10 * 2 + 15 / 3) * 2 + -10", Object::Integer(50)),
  ];

  for test in &tests {
    let (input, result) = test;
    let result_object = test_eval(input);
    println!("input: {}, result: {:?}", input, result);
    test_result(&result_object, result);
  }

  Ok(())
}

#[test]
fn test_eval_boolean_expression() -> Result<(), String> {
  let tests = vec![
    ("true", Object::Boolean(true)),
    ("false", Object::Boolean(false)),
    ("true == false", Object::Boolean(false)),
    ("true == true", Object::Boolean(true)),
    ("false == true", Object::Boolean(false)),
    ("false == false", Object::Boolean(true)),
    ("true != false", Object::Boolean(true)),
    ("true != true", Object::Boolean(false)),
    ("false != true", Object::Boolean(true)),
    ("false != false", Object::Boolean(false)),
    ("1 < 1", Object::Boolean(false)),
    ("1 < 2", Object::Boolean(true)),
    ("2 < 1", Object::Boolean(false)),
    ("1 > 1", Object::Boolean(false)),
    ("1 > 2", Object::Boolean(false)),
    ("2 > 1", Object::Boolean(true)),
    ("1 == 1", Object::Boolean(true)),
    ("1 == 2", Object::Boolean(false)),
    ("1 != 1", Object::Boolean(false)),
    ("1 != 2", Object::Boolean(true)),
  ];

  for test in &tests {
    let (input, result) = test;
    let result_object = test_eval(input);
    test_result(&result_object, result);
  }

  Ok(())
}

#[test]
fn test_eval_bang_operator() -> Result<(), String> {
  let tests = vec![
    ("!true", Object::Boolean(false)),
    ("!false", Object::Boolean(true)),
    ("!5", Object::Boolean(false)),
    ("!0", Object::Boolean(true)),
    ("!!true", Object::Boolean(true)),
    ("!!false", Object::Boolean(false)),
    ("!!5", Object::Boolean(true)),
    ("!!0", Object::Boolean(false)),
  ];

  for test in &tests {
    let (input, result) = test;
    let result_object = test_eval(input);
    println!("input: {}, result: {:?}", input, result);
    test_result(&result_object, result);
  }

  Ok(())
}

#[test]
fn test_eval_if_expression() -> Result<(), String> {
  let tests = vec![
    ("if (true) { 10 }", Object::Integer(10)),
    ("if (false) { 10 }", Object::Null),
    ("if (1) { 10 }", Object::Integer(10)),
    ("if (0) { 10 }", Object::Null),
    ("if (1 < 2) { 10 }", Object::Integer(10)),
    ("if (1 > 2) { 10 }", Object::Null),
    ("if (1 < 2) { 10 } else { 20 }", Object::Integer(10)),
    ("if (1 > 2) { 10 } else { 20 }", Object::Integer(20)),
  ];

  for test in &tests {
    let (input, result) = test;
    let result_object = test_eval(input);
    println!("input: {}, result: {:?}", input, result);
    test_result(&result_object, result);
  }

  Ok(())
}

#[test]
fn test_eval_return_statements() -> Result<(), String> {
  let tests = vec![
    ("return 10", Object::Integer(10)),
    ("return 10; 9", Object::Integer(10)),
    ("return 2 * 5; 9", Object::Integer(10)),
    ("9; return 2 * 5; 9", Object::Integer(10)),
  ];

  for test in &tests {
    let (input, result) = test;
    let result_object = test_eval(input);
    println!("input: {}, result: {:?}", input, result);
    test_result(&result_object, result);
  }

  Ok(())
}

#[test]
fn test_let_statements() -> Result<(), String> {
  let tests = vec![
    ("let a = 5; a;", Object::Integer(5)),
    ("let a = 5 * 5; a;", Object::Integer(25)),
    ("let a = 5; let b = a; b;", Object::Integer(5)),
    ("let a = 5; let b = a; let c = a + b + 5; c;", Object::Integer(15)),
  ];

  for test in &tests {
    let (input, result) = test;
    let result_object = test_eval(input);
    println!("input: {}, result: {:?}", input, result);
    test_result(&result_object, result);
  }

  Ok(())
}

#[test]
fn test_error_handling() -> Result<(), String> {
  let tests = vec![
    ("5 + true", "Expected integer, found: Boolean(true)"),
    ("5 + true; 5", "Expected integer, found: Boolean(true)"),
    ("-true", "Expected integer, found: Boolean(true)"),
    ("false + true", "Unknown operation: Boolean + Boolean"),
    ("5; false + true; 5", "Unknown operation: Boolean + Boolean"),
    ("if (10 > 1) { false + true; }", "Unknown operation: Boolean + Boolean"),
    ("
      if (10 > 1) {
        if (10 > 9) {
          return false + true;
        }

        return 1
      }
    ", "Unknown operation: Boolean + Boolean"),
    ("foobar", "Unknown identifier: foobar"),
    ("let foobar = 1; let foobar = 2;", "Identifier has already been declared: foobar"),
  ];

  for test in &tests {
    let (input, expected_error_message) = test;
    let result_object = test_eval(input);
    println!("input: {}, expected error: {:?}", input, expected_error_message);

    let error = match_or_fail!(result_object, Err(m) => m);
    let error_message = match_or_fail!(error, EvalError(m) => m);
    assert_eq!(&error_message, expected_error_message)
  }

  Ok(())
}

fn test_eval(input: &str) -> Result<Object, EvalError> {
  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = match_or_fail!(parser.parse_program(), Ok(m) => m);
  let env = Rc::new(RefCell::new(Environment::new()));
  eval(&program, &env)
}

fn test_result(actual_result: &Result<Object, EvalError>, expected_result: &Object) {
  let actual_result_value = match_or_fail!(actual_result, Ok(m) => m);

  match actual_result_value {
    Object::Integer(actual_integer) => {
      match expected_result {
        Object::Integer(expected_integer) => assert_eq!(actual_integer, expected_integer),
        x => panic!("Expected:\n\t{:?}\nGot:\n\t{:?}", expected_result, actual_result_value)
      }
    },
    Object::Boolean(actual_boolean) => {
      match expected_result {
        Object::Boolean(expected_boolean) => assert_eq!(actual_boolean, expected_boolean),
        x => panic!("Expected:\n\t{:?}\nGot:\n\t{:?}", expected_result, actual_result_value)
      }
    },
    Object::Null => {
      match expected_result {
        Object::Null => {},
        x => panic!("Expected:\n\t{:?}\nGot:\n\t{:?}", expected_result, actual_result_value)
      }
    },
    _ => panic!("Expected:\n\t{:?}\nGot:\n\t{:?}", expected_result, actual_result_value)
  }
}
