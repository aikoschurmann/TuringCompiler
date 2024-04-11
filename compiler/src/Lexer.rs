use std::collections::HashSet;


#[derive(Debug, Default, PartialEq, Clone)]
pub enum TokenKind {
    Number,
    Operator,
    Invalid,
    Symbol,
    Keyword,
    Comment,
    OpenParen,
    CloseParen,
    OpenParenCurly,
    CloseParenCurly,
    LineBreak,
    #[default]
    EOF
}

#[derive(Debug, Default, Clone)]
pub struct Position {
    pub row : usize,
    pub col : usize
}

#[derive(Debug, Default, Clone)]
pub struct Token {
    pub kind : TokenKind,
    pub text : String,
    pub length : usize,
    pub position : Position
}

pub struct Lexer {
    content: String,
    content_length: usize,
    cursor : usize, //absolute
    line : usize, 
    bol : usize, //beginning of line
    keywords: HashSet<String>, // Store keywords in a HashSet
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
            keywords: HashSet::new()
        };

        // Add keywords to the HashSet
        let keywords = ["if", "else", "while", "let", "define",];
        lexer.keywords.extend(keywords.iter().map(|s| s.to_string()));

        lexer
    }

    pub fn create_token(&self, kind: TokenKind, start: usize, length: usize) -> Token {
        let text = self.content[start..(start + length as usize)].to_string();
        let mut token = Token::default();
        token.text = text;
        token.position.col = start - self.bol + 1;
        token.position.row = self.line;
        token.length = length;
        token.kind = kind;
        token
    }

    fn is_whitespace(&self, x: char) -> bool {
        x.is_whitespace()
    }

    fn skip_whitespace(&mut self) {
        while self.cursor < self.content_length && self.is_whitespace(self.content.chars().nth(self.cursor as usize).unwrap()) {
            let x = self.content.chars().nth(self.cursor as usize).unwrap();
            self.cursor += 1;

            if x == '\n' {
                self.line += 1;
                self.bol = self.cursor;
            }  
        }
    }
    

    fn is_symbol_start(&self, x : char) -> bool {
        return x.is_alphabetic() || x == '_';
        
    }

    fn is_symbol(&self, x : char) -> bool {
        return x.is_ascii_alphanumeric() || x == '_';
    }

    fn is_number_start(&self, x: char) -> bool {
        x.is_ascii_digit() || x == '.'
    }

    fn is_number(&self, x: char) -> bool {
        x.is_ascii_digit() || x == '.' || x == 'e' || x == 'E'
    }
    
    // Helper method to advance cursor and token length
    fn advance_cursor(&mut self, length: u16) {
        self.cursor += length as usize;
    }

    // Helper method to handle symbol token
    fn handle_symbol(&mut self, start: usize) -> Token {
        while self.cursor < self.content_length && self.is_symbol(self.content.chars().nth(self.cursor).unwrap()) {
            self.cursor += 1;
        }
        let length = self.cursor - start;
        let text = self.content[start..(start + length as usize)].to_string();
        if self.keywords.contains(&text) {
            return self.create_token(TokenKind::Keyword, start, length)
        }
        return self.create_token(TokenKind::Symbol, start, self.cursor - start)
        
    }

    // Helper method to handle number token
    fn handle_number(&mut self, start: usize) -> Token {
        while self.cursor < self.content_length && self.is_number(self.content.chars().nth(self.cursor).unwrap()) {
            self.cursor += 1;
        }
        self.create_token(TokenKind::Number, start, self.cursor - start)
    }

    fn match_operator(&mut self, start: usize, op: char) -> Token {
        self.advance_cursor(1);

        if self.cursor < self.content_length && self.content.chars().nth(self.cursor).unwrap() == '=' {
            self.advance_cursor(1);
            self.create_token(TokenKind::Operator, start, 2)
        } else if op == '+' || op == '-' {
            if self.cursor < self.content_length && self.content.chars().nth(self.cursor).unwrap() == op {
                self.advance_cursor(1);
                self.create_token(TokenKind::Operator, start, 2)
            } else {
                self.create_token(TokenKind::Operator, start, 1)
            }
        } else {
            self.create_token(TokenKind::Operator, start, 1)
        }
    }

    // Main logic to process the next token
    pub fn next(&mut self) -> Token {
        // Skip whitespace calculate newlines
        self.skip_whitespace();
    
        // Check if past end of input EOF
        if self.cursor >= self.content_length {
            return Token::default();
        }
    
        let start = self.cursor;
        let current_char = self.content.chars().nth(start).unwrap(); // Store the current character
    
        match current_char {
            '#' => {
                while self.cursor < self.content_length && current_char != '\n' {
                    self.advance_cursor(1);
                }
                self.create_token(TokenKind::Comment, start, self.cursor - start)
            }
            '(' | ')' | '{' | '}' | ';' => {
                self.cursor += 1;
                match current_char {
                    '(' => self.create_token(TokenKind::OpenParen, start, 1),
                    ')' => self.create_token(TokenKind::CloseParen, start, 1),
                    '{' => self.create_token(TokenKind::OpenParenCurly, start, 1),
                    '}' => self.create_token(TokenKind::CloseParenCurly, start, 1),
                    ';' => self.create_token(TokenKind::LineBreak, start, 1),
                    _ => unreachable!(),
                }
            }

            '+' | '-' | '*' | '/' | '=' | '<' | '>' => {
                self.match_operator(start, current_char)
            }
            
            _ => {
                if self.is_symbol_start(current_char) {
                    self.handle_symbol(start)
                } else if self.is_number_start(current_char) {
                    self.handle_number(start)
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
                break;
            }
            tokens.push(token);
        }

        tokens
    }
}
