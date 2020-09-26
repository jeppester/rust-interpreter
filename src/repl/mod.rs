use std::io;
use std::io::prelude::*;
use std::process;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::eval::eval;
use crate::object::environment::*;
use std::rc::Rc;
use std::cell::RefCell;

pub fn start() {
  let stdin = io::stdin();
  let mut stdout = io::stdout();
  let env: WrappedEnv = Rc::new(RefCell::new(Environment::new()));

  loop {
    if write!(&mut stdout, ">> ").is_err() {
      process::exit(1)
    };
    if stdout.flush().is_err() {
      process::exit(1)
    };

    let mut input = String::new();
    if stdin.read_line(&mut input).is_err() {
      process::exit(1)
    };

    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);

    let program_result = parser.parse_program();

    match program_result {
      Err(_error) => continue,
      Ok(program) => {
        let eval_result = eval(&program, &env);

        match eval_result {
          Err(error) => {
            println!("Evaluation error {:?}", error);
            continue
          },
          Ok(object) => println!("{}", object.inspect())
        }
      },
    }
  }
}
