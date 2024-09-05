use crate::lexer::Token;
struct Parser {
    tokens: Vec<Token>,
    pos: isize,
}
impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
        }
    }
    
}
