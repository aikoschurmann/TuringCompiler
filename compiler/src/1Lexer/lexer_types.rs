use serde::Serialize;

#[derive(Debug, PartialEq, Default, Clone, Serialize)]
pub enum TokenKind {
    Number,
    Operator,
    Symbol,
    Keyword,
    Comment,
    OpenParen,
    CloseParen,
    OpenParenCurly,
    CloseParenCurly,
    LineBreak,
    NewLine,
    Invalid,
    StringLiteral,
    Ellipsis,
    Colon,
    Comma,
    #[default]
    EOF
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Position {
    pub row : usize,
    pub col : usize
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Token {
    pub kind : TokenKind,
    pub text : String,
    pub length : usize,
    pub position : Position
}


