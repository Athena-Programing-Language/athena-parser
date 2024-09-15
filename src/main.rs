mod lexer;
mod parser;

use lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("log(1 + 2)".to_string());
    let tokens = lexer.tokenize();

    for token in tokens {
        println!("Found token: {:?}", token);
    }
}
