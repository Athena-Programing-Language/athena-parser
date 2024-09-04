mod lexer;
mod parser;

use lexer::Lexer;

fn main() {
    let input = "5 + 5
".to_string();
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    for token in tokens {
        println!("Found token: {:?}, name: {}", token.token_type, token.name);
    }
}
