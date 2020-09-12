use crate::eval::*;
use crate::lexer::*;
use crate::parser::*;
use crate::object::*;
use crate::parser::parser_error::ParserError;

#[test]
fn test_eval_integer_expression() -> Result<(), ParserError> {
  let tests = vec![
    ("5", "5"),
    ("10", "10"),
  ];

  for test in &tests {
    let (input, result) = test;
    let result_object = test_eval(input)?;
    test_integer_object(&result_object, result);
  }

  Ok(())
}

fn test_eval(input: &str) -> Result<Box<dyn Object>, ParserError> {
  let lexer = Lexer::new(input);
  let mut parser = Parser::new(lexer);
  let program = parser.parse_program()?;

  Ok(eval(&program))
}

fn test_integer_object(object: &Box<dyn Object>, result: &str) {
  match_or_fail!(object.get_type(), ObjectType::Integer => ());
  assert_eq!(object.inspect(), result);
}
