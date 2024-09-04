use std::ptr::null;

#[derive(Debug, Clone)]
pub(crate) enum TokenType {
    // Operators
    PLUS,
    MINUS,
    STAR,
    SLASH,
    EQUAL,        // For '='
    COLON_END,
    FUNCTION,
    // Keywords
    LET,
    CONST,
    IF,
    ELSE,

    // Types
    STRING_TYPE,  // For '$str'
    NUMBER_TYPE,  // For '$nbr'
    BOOL_TYPE,    // For '$bool'

    // Literals
    VARIABLE,
    STRING_LITERAL,
    NUMBER_LITERAL,
    BOOL_LITERAL,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub name: String,
}

impl Token {
    // Constructor for Token
    pub fn new(token_type: TokenType, name: String) -> Self {
        Token {
            token_type,
            name,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lexer {
    pub token_list: Vec<Token>,
    pub input: String,
    pub pos: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            token_list: Vec::new(),
            input,
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        while self.pos < self.input.len() {
            self.skip_whitespace();
            if let Some(token) = self.next_token() {
                self.token_list.push(token);
            }
        }
        self.token_list.clone()
    }

    fn next_token(&mut self) -> Option<Token> {
        let remaining_input = &self.input[self.pos..];

        if remaining_input.starts_with("let") {
            let token = Token::new(TokenType::LET, "let".to_string());
            self.pos += 3;
            return Some(token);
        } else if remaining_input.starts_with("cst") {
            let token = Token::new(TokenType::CONST, "cnt".to_string());
            self.pos += 3;
            return Some(token);
        } else if remaining_input.starts_with("+") {
            let token = Token::new(TokenType::PLUS, "+".to_string());
            self.pos += 1;
            return Some(token);
        } else if remaining_input.starts_with("-") {
            let token = Token::new(TokenType::MINUS, "-".to_string());
            self.pos += 1;
            return Some(token);
        } else if remaining_input.starts_with("*") {
            let token = Token::new(TokenType::STAR, "*".to_string());
            self.pos += 1;
            return Some(token);
        } else if remaining_input.starts_with("/") {
            let token = Token::new(TokenType::SLASH, "/".to_string());
            self.pos += 1;
            return Some(token);
        } else if remaining_input.starts_with("=") {
            let token = Token::new(TokenType::EQUAL, "=".to_string());
            self.pos += 1;
            return Some(token);
        }
        else if remaining_input.starts_with(char::is_numeric) || remaining_input.starts_with('.') {
            return Some(self.tokenize_number());
        }

        None
    }

    fn tokenize_number(&mut self) -> Token {
        let mut number = String::new();
        let mut has_dot = false;

        while self.pos < self.input.len() {
            let current_char = self.input[self.pos..].chars().next().unwrap();

            if current_char.is_digit(10) {
                number.push(current_char);
            } else if current_char == '.' && !has_dot {
                number.push(current_char);
                has_dot = true;
            } else {
                break;
            }
            self.pos += 1;
        }

        Token::new(TokenType::NUMBER_LITERAL, number)
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos..].chars().next().unwrap().is_whitespace() {
            self.pos += 1;
        }
    }
}
