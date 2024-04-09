enum TokenKind {
    Number,
    Operator,
    Invalid
}

struct Position {
    row : u16,
    col : u16
}

struct Token {
    kind : TokenKind,
    text : String,
    length : u8
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

    fn next() -> Token {
        token Token = {0}
        return token
    }
}


fn main() {
    println!("Lexing!");
}