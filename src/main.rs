mod lexer;
mod token;

use lexer::Lexer;
use std::io;
use std::io::prelude::*;
use std::process;

fn main() {
  let stdin = io::stdin();
  let mut stdout = io::stdout();
  use token::token_types::*;

  println!("Type in commands and see how they get parsed by the lexer");

  loop {
    if write!(&mut stdout, ">> ").is_err() { process::exit(1) };
    if stdout.flush().is_err() { process::exit(1) };

    let mut input = String::new();
    if stdin.read_line(&mut input).is_err() { process::exit(1) };

    let mut lexer = Lexer::new(&input);

    loop {
      let token = lexer.next_token();

      if token.token_type == EOF {
        break;
      }

      let literal = token.literal.unwrap();
      println!("Type: {}, Char: {}", token.token_type, literal);
    }
  }
}
