mod lexer;
mod token;
mod repl;

fn main() {
  println!("Type in commands and see how they get parsed by the lexer");
  repl::start()
}
