use std::fs::File;
use std::io::{self, Read};
mod lexer;


use lexer::Lexer;
use crate::lexer::Token;

fn main() -> io::Result<()> {
    let file_path = "/home/kovalenko/my-app/athena/src/athena-parser/src/input.at";
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let mut lexer = Lexer::new(content.to_string());
    let tokens: Vec<Token> = lexer.tokenize();
    for token in tokens {
        println!("Found token: {:?}", token);
    }
    Ok(())
}
