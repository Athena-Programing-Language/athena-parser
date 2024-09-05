mod lexer;
mod parser;

use lexer::Lexer;

fn main() {
    let input = "let a = 12 $nbr;".to_string();
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();

    for token in tokens {
        println!("Found token: {:?}", token.token_type, );
    }

}
