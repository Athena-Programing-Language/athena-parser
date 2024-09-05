use crate::lexer::{Token, TokenType};
use std::any::Any;
use crate::lexer::Lexer;
struct Parser {
    tokens: Vec<Token>,
    pos: isize,
    scope: Option<Box<dyn Any>>, // Використовуємо Option<Box<dyn Any>>
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            scope: None, // Ініціалізація як None
        }
    }
    fn twing(&self, _ext: TokenType) -> Option<&Token> {
        if(self.pos as usize) < self.tokens.len() {
            let current_token: Token<> = self.tokens[self.pos];
            if  {  }
        } else {
            None
        }
    }
}