mod test_support;

mod token;
mod ast;
mod object;

mod lexer;
mod parser;
mod eval;

mod repl;

fn main() {
  println!("Type in commands and see how they get parsed by the lexer");
  repl::start()
}
