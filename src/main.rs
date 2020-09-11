mod test_support;

mod token;
mod lexer;
mod parser;
mod ast;

mod repl;
mod object;
mod evaluator;

fn main() {
  println!("Type in commands and see how they get parsed by the lexer");
  repl::start()
}
