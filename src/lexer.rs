#[derive(Debug, Clone)]
pub(crate) enum TokenType {
    // Operators
    PLUS,
    MINUS,
    STAR,
    SLASH,
    EQUAL,
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
    DID,
    LOG,
    // Types
    STRING_TYPE,
    NUMBER_TYPE,
    BOOL_TYPE,
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
        self.token_list.push(Token::new(TokenType::EOF, "".to_string()));
        self.token_list.clone()
    }

    fn next_token(&mut self) -> Option<Token> {
        let remaining_input = &self.input[self.pos..];
         if remaining_input.starts_with("->") {
            self.pos += 2;
            return Some(Token::new(TokenType::RETURN, "->".to_string()));
        }
        else if remaining_input.starts_with("let") {
            self.pos += 3;
            return Some(Token::new(TokenType::LET, "let".to_string()));
        } else if remaining_input.starts_with("cst") {
            self.pos += 3;
            return Some(Token::new(TokenType::CONST, "cst".to_string()));
        } else if remaining_input.starts_with("+") {
            self.pos += 1;
            return Some(Token::new(TokenType::PLUS, "+".to_string()));
        }
        else if remaining_input.starts_with("-") {
            self.pos += 1;
            return Some(Token::new(TokenType::MINUS, "-".to_string()));
        } else if remaining_input.starts_with("*") {
            self.pos += 1;
            return Some(Token::new(TokenType::STAR, "*".to_string()));
        } else if remaining_input.starts_with("/") {
            self.pos += 1;
            return Some(Token::new(TokenType::SLASH, "/".to_string()));
        } else if remaining_input.starts_with("=") {
            self.pos += 1;
            return Some(Token::new(TokenType::EQUAL, "=".to_string()));
        } else if remaining_input.starts_with(";") || remaining_input.starts_with(",") {
            self.pos += 1;
            return Some(Token::new(TokenType::COLON_END, ";".to_string()));
        } else if remaining_input.starts_with(":end") {
            self.pos += 4;
            return Some(Token::new(TokenType::COLON_END, ";".to_string()));
        } else if remaining_input.starts_with("if:") {
            self.pos += 3;
            return Some(Token::new(TokenType::IF, "if:".to_string()));
        } else if remaining_input.starts_with(":else:") {
            self.pos += 6;
            return Some(Token::new(TokenType::ELSE, ":else:".to_string()));
        } else if remaining_input.starts_with("do") {
            self.pos += 2;
            return Some(Token::new(TokenType::DO, "do".to_string()));
        } else if remaining_input.starts_with("did") {
            self.pos += 3;
            return Some(Token::new(TokenType::DID, String::from("did".to_string())));
        } else if remaining_input.starts_with("log") {
            self.pos += 3;
            return  Some(Token::new(TokenType::LOG, "log".to_string()));
        }
        else if remaining_input.starts_with("while:") {
            self.pos += 6;
            return Some(Token::new(TokenType::WHILE, "while:".to_string()));
        } else if remaining_input.starts_with("for:") {
            self.pos += 4;
            return Some(Token::new(TokenType::FOR, "for:".to_string()));
        } else if remaining_input.starts_with("match:") {
            self.pos += 6;
            return Some(Token::new(TokenType::MATCH, "match:".to_string()));
        }  else if remaining_input.starts_with("(") {
            self.pos += 1;
            return Some(Token::new(TokenType::LPAD, "(".to_string()));
        } else if remaining_input.starts_with(")") {
            self.pos += 1;
            return Some(Token::new(TokenType::RPAD, ")".to_string()));
        } else if remaining_input.starts_with("&true") || remaining_input.starts_with("&false") {
            let token_type = if remaining_input.starts_with("&true") {
                TokenType::BOOL_LITERAL
            } else {
                TokenType::BOOL_LITERAL
            };
            let length = if remaining_input.starts_with("&true") { 5 } else { 6 };
            self.pos += length;
            return Some(Token::new(token_type, remaining_input[..length].to_string()));
        } else if remaining_input.starts_with('"') {
            return Some(self.tokenize_string_literal());
        } else if remaining_input.starts_with("{") {
            self.pos += 1;
            return Some(Token::new(TokenType::RSCR, "{".to_string()));
        } else if remaining_input.starts_with("}") {
            self.pos += 1;
            return Some(Token::new(TokenType::LSCR, "}".to_string()));
        } else if remaining_input.starts_with("$nbr") {
            self.pos += 4;
            return Some(Token::new(TokenType::NUMBER_TYPE, "$nbr".to_string()));
        } else if remaining_input.starts_with("$str") {
            self.pos += 4;
            return Some(Token::new(TokenType::STRING_TYPE, "$str".to_string()));
        } else if remaining_input.starts_with("$bool") {
            self.pos += 5;
            return Some(Token::new(TokenType::BOOL_TYPE, "$bool".to_string()));
        } else if remaining_input.chars().next().unwrap().is_digit(10) || remaining_input.starts_with('.') {
            return Some(self.tokenize_number());
        } else if remaining_input.chars().next().unwrap().is_alphabetic() || remaining_input.starts_with('_') {
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
        self.pos += 1; // Skip opening quote

        while self.pos < self.input.len() {
            let current_char = self.input[self.pos..].chars().next().unwrap();

            if current_char == '"' {
                self.pos += 1; // Skip closing quote
                break;
            } else if current_char == '\\' {
                // Handle escape sequences
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
