#[derive(Debug, Default, PartialEq)]
enum TokenKind {
    Number,
    Operator,
    Invalid,
    Symbol,
    #[default]
    EOF
}

#[derive(Debug, Default)]
struct Position {
    row : u16,
    col : u16
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
    line : u16, 
    bol : usize //beginning of line
}


impl Lexer {
    fn new(content : String) -> Lexer {
        let content_length = content.len() as usize;
        Lexer {
            content,
            content_length,
            cursor : 0,
            line: 0,
            bol : 0
        }
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
    fn next(&mut self) -> Token {
        let mut token = Token::default();

        // Skip whitespace calculate newlines
        self.skip_whitespace();

        // Check if past end of input
        if self.cursor >= self.content_length {
            return token;
        }

        let start = self.cursor;

        // Check for symbol
        if self.is_symbol_start(self.content.chars().nth(self.cursor).unwrap()) {
            while self.cursor < self.content_length && self.is_symbol(self.content.chars().nth(self.cursor).unwrap()) {
                self.cursor += 1;
                token.length += 1;
            }
            let text = self.content[start..self.cursor].to_string();
            token.text = text;
            token.kind = TokenKind::Symbol;
            return token
        }

        // Check for number
        if self.is_number_start(self.content.chars().nth(self.cursor).unwrap()) {
            while self.cursor < self.content_length && self.is_number(self.content.chars().nth(self.cursor).unwrap()) {
                self.cursor += 1;
                token.length += 1;
            }
            let text = self.content[start..self.cursor].to_string();
            token.text = text;
            token.kind = TokenKind::Number;
            return token
        }
        if self.cursor < self.content_length {
            self.cursor += 1;
            token.length += 1;
            let text = self.content.chars().nth(start).unwrap().to_string();
            token.text = text;
            token.kind = TokenKind::Invalid;
            return token
        }

        return token

    }
}


fn main() {
    println!("Lexing!");
    let mut lexer = Lexer::new("test 123 _variable <>".to_string());
    loop {
        let token = lexer.next();
        if token.kind == TokenKind::EOF {
            break;
        }
        println!("{:?}", token);
    }
}