use std::collections::HashSet;
use std::fs;

#[derive(Debug, Default, PartialEq)]
enum TokenKind {
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
    #[default]
    EOF
}

#[derive(Debug, Default)]
struct Position {
    row : usize,
    col : usize
}

#[derive(Debug, Default)]
struct Token {
    kind : TokenKind,
    text : String,
    length : u16,
    position : Position
}

struct Lexer {
    content: String,
    content_length: usize,
    cursor : usize, //absolute
    line : usize, 
    bol : usize, //beginning of line
    keywords: HashSet<String>, // Store keywords in a HashSet
}


impl Lexer {
    fn new(content : String) -> Lexer {
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
          lexer.keywords.insert(String::from("if"));
          lexer.keywords.insert(String::from("else"));
          lexer.keywords.insert(String::from("while"));
          lexer.keywords.insert(String::from("let"));
          lexer.keywords.insert(String::from("define"));


          lexer
    }

    fn create_token(&self, kind: TokenKind, start: usize, length: u16) -> Token {
        let text = self.content[start..(start + length as usize)].to_string();
        let mut token = Token::default();
        token.text = text;
        token.position.col = start - self.bol;
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
            return self.create_token(TokenKind::Keyword, start, length as u16)
        }
        return self.create_token(TokenKind::Symbol, start, (self.cursor - start) as u16)
        
    }

    // Helper method to handle number token
    fn handle_number(&mut self, start: usize) -> Token {
        while self.cursor < self.content_length && self.is_number(self.content.chars().nth(self.cursor).unwrap()) {
            self.cursor += 1;
        }
        self.create_token(TokenKind::Number, start, (self.cursor - start) as u16)
    }

    // Main logic to process the next token
    fn next(&mut self) -> Token {
        // Skip whitespace calculate newlines
        self.skip_whitespace();

        // Check if past end of input EOF
        if self.cursor >= self.content_length {
            return Token::default();
        }

        let start = self.cursor;

        match self.content.chars().nth(self.cursor).unwrap() {
            '#' => {
                while self.cursor < self.content_length && self.content.chars().nth(self.cursor).unwrap() != '\n' {
                    self.advance_cursor(1)
                }
                self.create_token(TokenKind::Comment, start, (self.cursor - start) as u16)
            }
            '(' => {
                self.advance_cursor(1);
                self.create_token(TokenKind::OpenParen, start, 1)
            }
            ')' => {
                self.advance_cursor(1);
                self.create_token(TokenKind::CloseParen, start, 1)
            }
            '{' => {
                self.advance_cursor(1);
                self.create_token(TokenKind::OpenParenCurly, start, 1)
            }
            '}' => {
                self.advance_cursor(1);
                self.create_token(TokenKind::CloseParenCurly, start, 1)
            }
            '=' | '<' | '>' => {
                self.advance_cursor(1);
                if self.cursor < self.content_length && self.content.chars().nth(self.cursor).unwrap() == '=' {
                    self.advance_cursor(1);
                    self.create_token(TokenKind::Operator, start, 2)
                } else {
                    self.create_token(TokenKind::Operator, start, 1)
                }
            }
            _ => {
                if self.is_symbol_start(self.content.chars().nth(self.cursor).unwrap()) {
                    self.handle_symbol(start)
                } else if self.is_number_start(self.content.chars().nth(self.cursor).unwrap()) {
                    self.handle_number(start)
                } else {
                    self.advance_cursor(1);
                    self.create_token(TokenKind::Invalid, start, 1)
                }
            }
        }
    }
}



fn main() {
    println!("Lexing!");

    let code = fs::read_to_string("input.txt").unwrap();

    let mut lexer = Lexer::new(code);
    loop {
        let token = lexer.next();
        if token.kind == TokenKind::EOF {
            break;
        }
        println!("{:?}", token);
    }
}