#[derive(Default)]
enum TokenKind {
    Number,
    Operator,
    Invalid,
    #[default]
    EOF
}

#[derive(Default)]
struct Position {
    row : u16,
    col : u16
}

#[derive(Default)]
struct Token {
    kind : TokenKind,
    text : String,
    length : u8,
    position : Position
}

struct Lexer {
    content: String,
    content_length: u8,
    cursor : u8, //absolute
    line : u8, 
    bol : u8 //beginning of line
}


impl Lexer {
    fn new(content : String, content_length : u8) -> Lexer {
        Lexer {
            content,
            content_length,
            cursor : 0,
            line: 0,
            bol : 0
        }
    }

    fn next(&self) -> Token {
        let token = Token::default();

        if self.cursor >= self.content_length {
            return Token {
                kind: TokenKind::EOF,
                length: 0,
                position: Position::default(),
                text: String::new(), // Empty string instead of null
            };
        }
        return token
    }
}


fn main() {
    println!("Lexing!");
}