use std::io;
use std::io::prelude::*;
use std::process;

use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn start() {
  let stdin = io::stdin();
  let mut stdout = io::stdout();

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
      Ok(program) => println!("{}", program.to_string()),
      Err(_error) => continue,
    }
  }
}
