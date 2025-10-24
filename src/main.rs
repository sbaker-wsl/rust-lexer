mod lexer;
mod tokens;
use std::io;

fn main() {
    println!("enter text to tokenize:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    let mut lex = lexer::Lexer::set_input(input);
    lex.print_tokens();
}
