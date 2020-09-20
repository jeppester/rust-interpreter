use crate::eval::*;
use crate::lexer::*;
use crate::parser::*;
use crate::object::*;

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

fn test_eval(input: &str) -> Object {
  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = match_or_fail!(parser.parse_program(), Ok(m) => m);
  match_or_fail!(eval(&program), Ok(m) => m)
}

fn test_result(actual_result: &Object, expected_result: &Object) {
  match actual_result {
    Object::Integer(actual_integer) => {
      match expected_result {
        Object::Integer(expected_integer) => assert_eq!(actual_integer, expected_integer),
        x => panic!("Expected:\n\t{:?}\nGot:\n\t{:?}", expected_result, actual_result)
      }
    },
    Object::Boolean(actual_boolean) => {
      match expected_result {
        Object::Boolean(expected_boolean) => assert_eq!(actual_boolean, expected_boolean),
        x => panic!("Expected:\n\t{:?}\nGot:\n\t{:?}", expected_result, actual_result)
      }
    },
    Object::Null => {
      match expected_result {
        Object::Null => {},
        x => panic!("Expected:\n\t{:?}\nGot:\n\t{:?}", expected_result, actual_result)
      }
    }
  }
}
