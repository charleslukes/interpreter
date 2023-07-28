use crate::{lexer::Lexer, token::EOF};
use std::io;

const PROMPT: &'static str = ">>";

pub fn start() -> io::Result<()> {
    println!("{}", PROMPT);
    let mut user_input = String::new();
    let stdin = io::stdin();
    let _res = stdin.read_line(&mut user_input);
    let mut token_type = String::new();

    let mut counter = 0;
    let mut lexer = Lexer::new(user_input.to_string());

    while counter <= user_input.len() && token_type != EOF {
        let l = lexer.next_token();
        println!("{:?}", l);
        token_type = l.token_type;
        counter += 1;
    }

    Ok(())
}
