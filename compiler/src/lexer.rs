use crate::lexer_types::*;
use std::collections::HashSet;

pub struct Lexer {
    content: String,
    content_length: usize,
    cursor : usize, //absolute
    line : usize, 
    bol : usize, //beginning of line
    last_token : Token,
    keywords: HashSet<String>, // Store keywords in a HashSet
    operators: HashSet<String>, // Store operators in a HashSet
}

impl Lexer {
    pub fn new(content : String) -> Lexer {
        let content_length = content.len() as usize;
        let mut lexer = Lexer {
            content,
            content_length,
            cursor : 0,
            line: 0,
            bol : 0,
            last_token : Token::default(),
            keywords: HashSet::new(),
            operators: HashSet::new()
        };

        let keywords = ["if", "else", "while", "let", "define"];
        let operators = ["or", "and"];

        lexer.keywords.extend(keywords.iter().map(|s| s.to_string()));
        lexer.operators.extend(operators.iter().map(|s| s.to_string()));

        lexer
    }

    fn not_end(&mut self) -> bool{
        return self.cursor < self.content_length;
    }

    fn is_whitespace_without_newline(&mut self, c: char) -> bool {
        c.is_whitespace() && c != '\n'
    }

    fn skip_whitespace(&mut self) {
        while self.not_end() && self.is_whitespace_without_newline(self.content.chars().nth(self.cursor as usize).unwrap()) {
            self.cursor += 1;
        }
    }
    // Helper method to advance cursor
    fn advance_cursor(&mut self, length: usize) {
        self.cursor += length as usize;
    }

    fn get_current_char(&mut self) -> char{
        let pos = self.cursor;
        return self.content.chars().nth(pos).unwrap();
    }

    pub fn create_token(&self, kind: TokenKind, position: usize, length: usize) -> Token {
        let text = self.content[position..(position + length)].to_string();
        let token = Token { kind: kind, text: text, length: length, position: Position {col : position - self.bol, row : self.line} };
        return token
    }

    fn match_operator(&mut self, start: usize, op: char) -> Token {
        self.advance_cursor(1);
        let current_char = self.get_current_char();
    
        let token = if self.not_end() && current_char == '=' {
            self.advance_cursor(1);
            self.create_token(TokenKind::Operator, start, 2)
        } else if op == '+' || op == '-' {
            if self.not_end() && current_char == op {
                self.advance_cursor(1);
                self.create_token(TokenKind::Operator, start, 2)
            } else {
                self.create_token(TokenKind::Operator, start, 1)
            }
        } else {
            self.create_token(TokenKind::Operator, start, 1)
        };
    
        self.last_token = token.clone();
        token
    }

    pub fn is_symbol_start(&self, x : char) -> bool {
        return x.is_alphabetic() || x == '_';
    }

    fn is_symbol(&self, x : char) -> bool {
        return x.is_ascii_alphanumeric() || x == '_';
    }

    // Helper method to handle symbol token
    fn handle_symbol(&mut self,) -> Token {
        let start = self.cursor;
    
        while self.not_end() && self.is_symbol(self.content.chars().nth(self.cursor).unwrap()) {
            self.advance_cursor(1);
        }
        let length = self.cursor - start;
    
        let text = self.content[start..(start + length as usize)].to_string();
        
        let token = if self.operators.contains(&text) {
            self.create_token(TokenKind::Operator, start, self.cursor - start)
        } else if self.keywords.contains(&text) {
            self.create_token(TokenKind::Keyword, start, length)
        } else {
            self.create_token(TokenKind::Symbol, start, self.cursor - start)
        };
        
        self.last_token = token.clone();
        token
    }
    
    fn is_decimal_point(&self, x: char) -> bool {
        x == '.'
    }

    fn is_number_seperator(&self, x: char) -> bool {
        x == '.' || x == 'e' || x == 'E'
    }

    fn is_number(&self, x: char) -> bool {
        x.is_ascii_digit()
    }

    // Helper method to handle number token
    fn handle_number(&mut self) -> Token {
        let start = self.cursor;
        let mut has_decimal_point = false;
        let mut has_seperator = false;
    
        while self.not_end() {
            let current_char = self.get_current_char();
    
            if self.is_number(current_char) {
                self.advance_cursor(1)
            } else if self.is_decimal_point(current_char){
                if has_decimal_point {
                    let error_message = format!("Invalid number: multiple decimals in number at row {}, column {}", self.line + 1, self.cursor - self.bol);
                    panic!("{}", error_message);
                }
                has_decimal_point = true;
                self.advance_cursor(1);
            } else if self.is_number_seperator(current_char) {
                if has_seperator {
                    let error_message = format!("Invalid number: multiple separators in number at row {}, column {}", self.line + 1, self.cursor - self.bol);
                    panic!("{}", error_message);
                }
                has_seperator = true;
                self.advance_cursor(1);
            } else {
                break;
            }
        }
    
        let token = self.create_token(TokenKind::Number, start, self.cursor - start);
        self.last_token = token.clone();
        token
    }
    fn handle_string_literal(&mut self) -> Token {
        let start = self.cursor;
        self.advance_cursor(1); // Skip opening double quote
        let mut escaped = false;
        let mut string_content = String::new();
        
        while self.not_end() {
            let current_char = self.get_current_char();
            self.advance_cursor(1);
            
            if escaped {
                match current_char {
                    '"' => string_content.push('"'), 
                    '\\' => string_content.push('\\'),
                    _ => {
                        panic!("Invalid escape sequence: \\{}", current_char);
                    }
                }
                escaped = false; 
            } else if current_char == '\\' {
                escaped = true; 
            } else if current_char == '"' && !escaped {
                let mut token = self.create_token(TokenKind::StringLiteral, start + 1, self.cursor - start - 2);
                token.text = string_content;
                return token
            } else {
                string_content.push(current_char);
            }
        }
        panic!("Unterminated string literal starting at position {}", start);
    }
    
    pub fn next(&mut self) -> Token {
        //note doesn't skip newline
        self.skip_whitespace();
        //remember a start for multi char tokens
        let start = self.cursor;
        //return EOF at end of input
        if self.cursor >= self.content_length {
            return self.create_token(TokenKind::EOF, start, 0)
        }

        let current_char = self.get_current_char();

        match current_char {
            '#' => {
                while self.not_end() && current_char != '\n' {
                    self.advance_cursor(1);
                }
                let token = self.create_token(TokenKind::Comment, start, self.cursor - start);
                self.last_token = token.clone();
                return token;
            }
            '(' => {
                self.cursor += 1;
                let token = self.create_token(TokenKind::OpenParen, start, 1);
                self.last_token = token.clone();
                token
            },
            ')' => {
                self.cursor += 1;
                let token = self.create_token(TokenKind::CloseParen, start, 1);
                self.last_token = token.clone();
                token
            },
            '{' => {
                self.cursor += 1;
                let token = self.create_token(TokenKind::OpenParenCurly, start, 1);
                self.last_token = token.clone();
                token
            },
            '}' => {
                self.cursor += 1;
                let token = self.create_token(TokenKind::CloseParenCurly, start, 1);
                self.last_token = token.clone();
                token
            },
            ';' => {
                self.cursor += 1;
                let token = self.create_token(TokenKind::LineBreak, start, 1);
                self.last_token = token.clone();
                token
            },
            ':' => {
                self.cursor += 1;
                let token = self.create_token(TokenKind::Colon, start, 1);
                self.last_token = token.clone();
                token
            },
            ',' => {
                self.cursor += 1;
                let token = self.create_token(TokenKind::Comma, start, 1);
                self.last_token = token.clone();
                token
            },
            '\n' => {
                self.cursor += 1;
                let token = self.create_token(TokenKind::NewLine, start, 1);
                self.line += 1;
                self.bol = self.cursor;
                token
            },

            '+' | '-' | '*' | '/' | '=' | '<' | '>' => {
                self.match_operator(start, current_char)
            }
            '"' => self.handle_string_literal(),
            '.' => {
                self.advance_cursor(1);
                let token = if self.not_end() && self.get_current_char() == '.' && self.last_token.kind == TokenKind::OpenParenCurly {
                    self.advance_cursor(1);
                    if self.not_end() && self.get_current_char() == '.' {
                        self.advance_cursor(1);
                        self.create_token(TokenKind::Ellipsis, start, 3)
                    } else {
                        let error_msg = format!("expected '...' istead of '..' at row {}, column {}", self.line + 1, start - self.bol + 1);

                        panic!("{}", error_msg);
                    }
                } else {
                    let error_msg = format!("expected '...' to be in an empty function body at row {}, column {}", self.line + 1, start - self.bol + 1);
                    panic!("{}", error_msg);
                };
                self.last_token = token.clone();
                token
            }
            _ => {
                if self.is_symbol_start(current_char) {
                    self.handle_symbol()
                } else if self.is_number(current_char) {
                    self.handle_number()
                } else {
                    self.advance_cursor(1);
                    self.create_token(TokenKind::Invalid, start, 1)
                }
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next();
            if token.kind == TokenKind::EOF {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        tokens
    }
}