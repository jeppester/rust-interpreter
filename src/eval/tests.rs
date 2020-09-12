use crate::eval::*;
use crate::lexer::*;
use crate::parser::*;
use crate::object::*;

#[test]
fn test_eval_integer_expression() -> Result<(), String> {
  let tests = vec![
    ("5", "5"),
    ("10", "10"),
  ];

  for test in &tests {
    let (input, result) = test;
    let result_object = test_eval(input);
    test_integer_object(&result_object, result);
  }

  Ok(())
}

fn test_eval(input: &str) -> Box<dyn Object> {
  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);

  let program = match_or_fail!(parser.parse_program(), Ok(m) => m);
  match_or_fail!(eval(&program), Ok(m) => m)
}

fn test_integer_object(object: &Box<dyn Object>, result: &str) {
  match_or_fail!(object.get_type(), ObjectType::Integer => ());
  assert_eq!(object.inspect(), result);
}
