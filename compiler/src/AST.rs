mod Lexer;

use Lexer::*;

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
    program: Vec<Node>,
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
    current_token : Option<Token>
}

impl Parser {
    fn advance(&mut self){
        if self.position < self.tokens.len() {
            self.current_token = Some(self.tokens[self.position].clone());
            self.position += 1;
        } else {
            self.current_token = None;
        }
    }

    fn new(tokens: Vec<Token>) -> Parser{
        let length = tokens.len() as usize;
        let mut parser = Parser {
            tokens,
            length,
            position : 0,
            current_token : None
        };

        parser
    }
}



fn main() {
    println!("Building a tree!");
}
