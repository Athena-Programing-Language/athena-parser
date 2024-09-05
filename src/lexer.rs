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
    LPAD,
    RPAD,
    LSCR,
    RSCR,
    // Keywords
    LET,
    CONST,
    IF,
    ELSE,
    DO,
    WHILE,
    FOR,
    MATCH,
    RETURN,
    // Types
    STRING_TYPE,  // For '$str'
    NUMBER_TYPE,  // For '$nbr'
    BOOL_TYPE,    // For '$bool'

    // Literals
    VARIABLE,
    STRING_LITERAL,
    NUMBER_LITERAL,
    BOOL_LITERAL,
    EOF,
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
            pos: 0,
            input,
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
            let token = Token::new(TokenType::CONST, "cst".to_string());
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
        } else if remaining_input.starts_with(";") || remaining_input.starts_with(",") {
            let token: Token = Token::new(TokenType::COLON_END, ";".to_string());
            self.pos += 1;
            return Some(token);
        } else if remaining_input.starts_with(":end") {
            let token: Token = Token::new(TokenType::COLON_END, ";".to_string());
            self.pos += 4;
            return Some(token);
        } else if remaining_input.starts_with("if:") {
            let token: Token = Token::new(TokenType::IF, "if:".to_string());
            self.pos += 3;
            return Some(token);
        } else if remaining_input.starts_with(":else:") {
            let token: Token = Token::new(TokenType::ELSE, ":else:".to_string());
            self.pos += 6;
            return Some(token);
        } else if remaining_input.starts_with("do") {
            let token: Token = Token::new(TokenType::DO, "do".to_string());
            self.pos += 2;
            return Some(token);
        } else if remaining_input.starts_with("while:") {
            let token: Token = Token::new(TokenType::WHILE, "while:".to_string());
            self.pos += 6;
            return Some(token);
        } else if remaining_input.starts_with("for:") {
            let token: Token = Token::new(TokenType::FOR, "for:".to_string());
            self.pos += 4;
            return Some(token);
        } else if remaining_input.starts_with("match:") {
            let token: Token = Token::new(TokenType::MATCH, "match:".to_string());
            self.pos += 6;
            return Some(token);
        } else if remaining_input.starts_with("->") {
            let token = Token::new(TokenType::RETURN, "->".to_string());
            self.pos += 2;
            return Some(token);
        } else if remaining_input.starts_with("(") {
            let token: Token = Token::new(TokenType::LPAD, "(".to_string());
            self.pos += 1;
            return Some(token);
        } else if remaining_input.starts_with(")") {
            let token: Token = Token::new(TokenType::RPAD, ")".to_string());
            self.pos += 1;
            return Some(token);
        } else if remaining_input.starts_with("&true") || remaining_input.starts_with("&false") {
            let token: Token = Token::new(TokenType::BOOL_LITERAL, remaining_input.to_string());
            if remaining_input.starts_with("&true") {
                self.pos += 5;
            } else {
                self.pos += 6;
            }
            return Some(token);
        } else if remaining_input.starts_with('"') {
            return Some(self.tokenize_string_literal());
        } else if remaining_input.starts_with("{") {
            let token: Token = Token::new(TokenType::RSCR, "{".to_string());
            self.pos += 1;
            return Some(token);
        } else if remaining_input.starts_with("}") {
            let token: Token = Token::new(TokenType::LSCR, "}".to_string());
            self.pos += 1;
            return Some(token);
        } else if remaining_input.starts_with("$nbr") {
            let token: Token = Token::new(TokenType::NUMBER_TYPE, "$nbr".to_string());
            self.pos += 4;
            return Some(token);
        } else if remaining_input.starts_with("$str") {
            let token: Token = Token::new(TokenType::STRING_TYPE, "$str".to_string());
            self.pos += 4;
            return Some(token);
        } else if remaining_input.starts_with("$bool") {
            let token: Token = Token::new(TokenType::BOOL_TYPE, "$bool".to_string());
            self.pos += 5;
            return Some(token);
        }
        else if remaining_input.chars().next().unwrap().is_digit(10) || remaining_input.starts_with('.') {
            return Some(self.tokenize_number());
        }
        else if remaining_input.chars().next().unwrap().is_alphabetic() || remaining_input.starts_with('_') {
            return Some(self.tokenize_variable());
        }
        None
    }
    fn tokenize_variable(&mut self) -> Token {
        let mut variable = String::new();

        while self.pos < self.input.len() {
            let current_char = self.input[self.pos..].chars().next().unwrap();
            if current_char.is_alphanumeric() || current_char == '_' {
                variable.push(current_char);
            } else {
                break;
            }
            self.pos += 1;
        }

        Token::new(TokenType::VARIABLE, variable)
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

    fn tokenize_string_literal(&mut self) -> Token {
        let mut literal = String::new();
        self.pos += 1; // Пропускаємо відкриття лапок

        while self.pos < self.input.len() {
            let current_char = self.input[self.pos..].chars().next().unwrap();

            if current_char == '"' {
                self.pos += 1; // Пропускаємо закриття лапок
                break;
            } else if current_char == '\\' {
                // Обробка екранованих символів
                self.pos += 1;
                let next_char = self.input[self.pos..].chars().next().unwrap();
                match next_char {
                    'n' => literal.push('\n'),
                    't' => literal.push('\t'),
                    '\\' => literal.push('\\'),
                    '"' => literal.push('"'),
                    _ => literal.push(next_char),
                }
            } else {
                literal.push(current_char);
            }
            self.pos += 1;
        }

        Token::new(TokenType::STRING_LITERAL, literal)
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos..].chars().next().unwrap().is_whitespace() {
            self.pos += 1;
        }
    }
}

