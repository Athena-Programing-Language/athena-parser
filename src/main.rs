mod lexer;
mod parser;

use lexer::Lexer;

fn main() {
    let mut lexer = Lexer::new("for: let a = 0; a = 5; a + 1; {a}".to_string());
    let tokens = lexer.tokenize();

    for token in tokens {
        println!("Found token: {:?}", token);
    }
}
