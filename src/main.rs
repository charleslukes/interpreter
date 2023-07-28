mod token;
mod lexer;
mod repl;


use crate::repl::start;
fn main() {
    println!("This is the programming language!");
    println!("Type code in command!");
    let _ = start();
}
