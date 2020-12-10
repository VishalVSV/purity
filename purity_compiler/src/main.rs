pub mod lexer;
pub mod utils;

use std::fs::read_to_string;

fn main() {
    let a = read_to_string("./sample.pty").unwrap();
    let lexer = lexer::Lexer::new(&a);

    for token in lexer {
        println!("{:?}",token);
    }
}