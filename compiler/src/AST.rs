mod lexer;

use std::fs;
use lexer::*;

#[derive(Debug)]
enum Node {
    Program(Program),
    FunctionDeclaration(FunctionDeclaration),
    VariableDeclaration(VariableDeclaration),
    Literal(Literal),
    Identifier(Identifier),
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    LogicalExpression(LogicalExpression),
}


#[derive(Debug)]
struct Program {
    program: Option<Vec<Node>>,
}

#[derive(Debug)]
struct Identifier {
    name: String,
}

#[derive(Debug)]
struct FunctionDeclaration {
    id: Identifier,
    params: Vec<Node>,
    body: Vec<Node>,
}

#[derive(Debug)]
struct VariableDeclaration {
    id: Identifier,
    init: Literal,
}

#[derive(Debug)]
struct Literal {
    raw: String,
}

#[derive(Debug)]
struct BinaryExpression {
    left: Box<Node>,
    right: Box<Node>,
    operator: Operator,
}

#[derive(Debug)]
enum LogicalOperator {
    Or,
    And,
}
#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Increment,
    AddEquals,
    MinusEquals,
    TimesEquals,
    DivideEquals,
    Equals,
    SmallerEquals,
    GreaterEquals,
    Smaller,
    Greater
}

#[derive(Debug)]
struct UnaryExpression {
    operand: Box<Node>,
    operator: Operator
}

#[derive(Debug)]
struct IfStatement {
    test: Box<Node>,
    consequent: Box<Node>,
    alternate: Option<Box<Node>>
}

#[derive(Debug)]
struct WhileStatement {
    test: Box<Node>,
    body: Box<Node>,
}

#[derive(Debug)]
struct LogicalExpression {
    operator : LogicalOperator,
    left : Box<Node>,
    right : Box<Node>
}

struct Parser {
    tokens : Vec<Token>,
    length : usize,
    position : usize,
}

impl Parser {
    fn advance(&mut self, length: usize){
        self.position += length;
    }
    fn handle_assignment(&self) {
        todo!()
    }
    fn handle_if(&self) {
        todo!()
    }
    fn handle_while(&self) {
        todo!()
    }
    fn handle_function_assignment(&self) {
        todo!()
    }

    fn handle_keyword(&self, tokens: &[Token]) {
        match tokens[0].text.as_str() {
            "let" => self.handle_assignment(),
            "if" => self.handle_if(),
            "while" => self.handle_while(),
            "define" => self.handle_function_assignment(),
            _ => unreachable!()
        }
    }
    

    fn parse(&mut self) -> Program{
        let mut program = Program { program: Some(Vec::new()) };
        
        while self.position < self.length {

            let start = self.position;

            match self.tokens[self.position].kind {
                TokenKind::Number => todo!(),
                TokenKind::Operator => todo!(),
                TokenKind::Invalid => todo!(),
                TokenKind::Symbol => todo!(),
                TokenKind::Keyword => {
                    while self.tokens[self.position].kind != TokenKind::LineBreak {
                        self.advance(1)
                    }
                
                    self.handle_keyword(&self.tokens[start..=self.position]);

                },
                TokenKind::Comment => todo!(),
                TokenKind::OpenParen => todo!(),
                TokenKind::CloseParen => todo!(),
                TokenKind::OpenParenCurly => todo!(),
                TokenKind::CloseParenCurly => todo!(),
                TokenKind::LineBreak => todo!(),
                TokenKind::EOF => todo!(),
            } 
        }

        program
    }

    fn new(tokens: Vec<Token>) -> Parser{
        let length = tokens.len() as usize;
        let mut parser = Parser {
            tokens,
            length,
            position : 0,
        };

        parser
    }
}



fn main() {
    println!("Lexing!");

    let code = fs::read_to_string("input.txt").unwrap();
    let mut lexer = Lexer::new(code);

    let tokens = lexer.tokenize();

    for token in &tokens {
        println!("{:?}", token);
    }

    println!("Building a tree!");
    let mut parser = Parser::new(tokens);

    let _ast = parser.parse();

}
