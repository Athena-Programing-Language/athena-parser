use crate::lexer::Token;
pub struct BinaryExpression {
    pub left: Box<Token>,
    pub operator: Token,
    pub right: Box<Token>,
}
impl BinaryExpression {
    pub fn new(left: Token, operator: Token, right: Token) -> Self {
        Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}