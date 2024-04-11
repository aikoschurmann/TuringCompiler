use std::fs;
use lexer::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;

use serde::Serialize;




impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_str = match self {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Increment => "++",
            Operator::AddEquals => "+=",
            Operator::MinusEquals => "-=",
            Operator::TimesEquals => "*=",
            Operator::DivideEquals => "/=",
            Operator::Equals => "==",
            Operator::SmallerEquals => "<=",
            Operator::GreaterEquals => ">=",
            Operator::Smaller => "<",
            Operator::Greater => ">",
            Operator::Exp => "^",
            Operator::OpenParen => "(",
            Operator::CloseParen => ")",
        };
        write!(f, "{}", op_str)
    }
}

impl fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let left_display = match self.left.as_ref() {
            Node::BinaryExpression(expr) => format!("{}", expr),
            Node::Literal(literal) => format!("{}", literal.raw),
            Node::Identifier(identifier) => format!("{}", identifier.name),
            _ => panic!("Invalid left node in BinaryExpression"),
        };

        let right_display = match self.right.as_ref() {
            Node::BinaryExpression(expr) => format!("{}", expr),
            Node::Literal(literal) => format!("{}", literal.raw),
            Node::Identifier(identifier) => format!("{}", identifier.name),
            _ => panic!("Invalid right node in BinaryExpression"),
        };

        write!(f, "({} {} {})", left_display, self.operator, right_display)
    }
}

#[derive(Debug, Serialize)]
pub enum Node {
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

#[derive(Debug, Serialize)]
pub enum InitExpression {
    Literal(Literal),
    Identifier(Identifier),
    BinaryExpression(BinaryExpression),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Precedence {
    Low,
    Medium,
    High,
}

fn precedence(op: &Operator) -> Precedence {
    match op {
        Operator::Add | Operator::Subtract | Operator::OpenParen  => Precedence::Low,
        Operator::Multiply | Operator::Divide => Precedence::Medium,
        Operator::Exp => Precedence::High,
        _ => panic!("Unknown operator"),
    }
}

#[derive(Debug, Serialize)]
pub struct Program {
    program: Vec<Node>,
}

#[derive(Debug, Serialize)]
pub struct Identifier {
    name: String,
}

#[derive(Debug, Serialize)]
pub struct FunctionDeclaration {
    id: Identifier,
    params: Vec<Node>,
    body: Vec<Node>,
}

#[derive(Debug, Serialize)]
pub struct VariableDeclaration {
    id: Identifier,
    init: InitExpression,
}

#[derive(Debug, Serialize)]
pub struct Literal {
    raw: String,
}

#[derive(Debug, Serialize)]
pub struct BinaryExpression {
    left: Box<Node>,
    right: Box<Node>,
    operator: Operator,
}

#[derive(Debug, Serialize)]
pub enum LogicalOperator {
    Or,
    And,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Operator {
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
    Greater,
    Exp,
    OpenParen,
    CloseParen
}

#[derive(Debug, Serialize)]
pub struct UnaryExpression {
    operand: Box<Node>,
    operator: Operator
}

#[derive(Debug, Serialize)]
pub struct IfStatement {
    test: Box<Node>,
    consequent: Box<Node>,
    alternate: Option<Box<Node>>
}

#[derive(Debug, Serialize)]
pub struct WhileStatement {
    test: Box<Node>,
    body: Box<Node>,
}

#[derive(Debug, Serialize)]
pub struct LogicalExpression {
    operator : LogicalOperator,
    left : Box<Node>,
    right : Box<Node>
}

pub struct Parser {
    tokens : Vec<Token>,
    length : usize,
    position : usize,
}

impl Parser {
    fn advance(&mut self, length: usize) {
        self.position += length;
    }

    fn validate_token(&self, token: Option<Token>, expected_kind: TokenKind, expected_text: Option<&str>, error_msg: &str, statement: &str) {
        match token {
            Some(token) => {
                if token.kind != expected_kind {
                    panic!("{} at row {}, column {}\n{}\n{}", error_msg, token.position.row, token.position.col, statement, self.underline_error(&statement, token.position.col, token.length));
                }
                if let Some(text) = expected_text {
                    if token.text != *text {
                        panic!("{} found unexpected symbol '{}' at row {}, column {}\n{}\n{}", error_msg, token.text, token.position.row, token.position.col, statement, self.underline_error(&statement, token.position.col, token.length));
                    }
                }
            }
            None => {
                panic!("{}", error_msg);
            }
        }
    }

    fn underline_error(&self, token_text: &str, error_col: usize, error_length: usize) -> String {
        let mut underline = String::new();
        for col in 0..token_text.len() {
            if col >= error_col && col < error_col + error_length {
                underline.push('^');
            } else {
                underline.push(' ');
            }
        }
        underline
    }

    fn get_nth_line(&self, file_path: &str, n: usize) -> String {
        let file = File::open(file_path).expect("Error: File not found.");
        let reader = BufReader::new(file);
        let line = reader.lines().nth(n).expect("Error: Line does not exist in the file.");
        line.expect("Error reading line").trim().to_string()
    }
    
    fn to_operator(&self, token_text: &str) -> Option<Operator> {
        match token_text {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Subtract),
            "*" => Some(Operator::Multiply),
            "/" => Some(Operator::Divide),
            "(" => Some(Operator::OpenParen),
            _ => None,
        }
    }
    
    fn handle_equation(&self, tokens: &[Token]) -> Node {
        let mut number_queue: Vec<Node> = Vec::new();
        let mut operator_stack: Vec<Operator> = Vec::new();
        let mut expect_operand = true;
        let current_line = tokens[0].position.row;
        let statement = self.get_nth_line("input.txt", current_line);
        let mut paren_count = 0; // Track the number of parentheses
    
        for token in tokens {
            match token.kind {
                // Handle number or symbol tokens
                TokenKind::Number | TokenKind::Symbol => {
                    if !expect_operand {
                        // Handle unexpected operand
                        let error_col = token.position.col;
                        let error_length = token.length;
                        let error_message = format!(
                            "Expected operator instead of '{}' at row {}, column {}",
                            token.text, token.position.row, error_col
                        );
                        panic!("{}\n{}\n{}", error_message, statement, self.underline_error(&error_message, error_col, error_length));
                    }
                    match token.kind {
                        TokenKind::Number => {
                            number_queue.push(Node::Literal(Literal { raw: token.text.parse().unwrap() }));
                            expect_operand = false;
                        },
                        TokenKind::Symbol => {
                            number_queue.push(Node::Identifier(Identifier { name: token.text.clone() }));
                            expect_operand = false;
                        },
                        _ => unreachable!()
                    }
                }
                // Handle operator tokens
                TokenKind::Operator => {
                    if expect_operand {
                        // Handle unexpected operator
                        let error_col = token.position.col;
                        let error_length = token.length;
                        let error_message = format!(
                            "Expected an operand instead of '{}' at row {}, column {}",
                            token.text, token.position.row, error_col
                        );
                        panic!("{}\n{}\n{}", error_message, statement, self.underline_error(&error_message, error_col, error_length));
                    }
                    let operator = match self.to_operator(&token.text) {
                        Some(op) => op,
                        None => {
                            let error_col = token.position.col;
                            let error_length = token.length;
                            let error_message = format!("Invalid operator '{}' at row {}, column {}", token.text, token.position.row, error_col);
                            panic!("{}\n{}\n{}", error_message, statement, self.underline_error(&error_message, error_col, error_length));
                        }
                    };
    
                    while !operator_stack.is_empty() && precedence(&operator) <= precedence(operator_stack.last().unwrap()) && *operator_stack.last().unwrap() != Operator::OpenParen {
                        // Pop operators from the stack and construct binary expressions
                        let last_operator = operator_stack.pop().unwrap();
                        let right = number_queue.pop().unwrap();
                        let left = number_queue.pop().unwrap();
                        let binary_expr = BinaryExpression {
                            left: Box::new(left),
                            right: Box::new(right),
                            operator: last_operator,
                        };
                        number_queue.push(Node::BinaryExpression(binary_expr));
                    }
                    operator_stack.push(operator); // Add the current operator to the stack
                    expect_operand = true;
                }
                // Handle opening parenthesis
                TokenKind::OpenParen => {
                    if !expect_operand {
                        // Handle unexpected opening parenthesis
                        let error_col = token.position.col;
                        let error_length = token.length;
                        let error_message = format!("Unexpected '('. Expected operator before parentheses at row {}, column {}", token.position.row, error_col);
                        panic!("{}\n{}\n{}", error_message, statement, self.underline_error(&error_message, error_col, error_length));
                    }
                    operator_stack.push(Operator::OpenParen);
                    paren_count += 1; // Increment the count of opening parentheses
                    expect_operand = true;
                }
                // Handle closing parenthesis
                TokenKind::CloseParen => {
                    paren_count -= 1; // Decrement the count of opening parentheses
                    if paren_count < 0 {
                        // Handle mismatched parentheses: too many closing parentheses
                        let error_col = token.position.col;
                        let error_length = token.length;
                        let error_message = format!("Mismatched parentheses: Too many closing parentheses at row {}, column {}", token.position.row, error_col);
                        panic!("{}\n{}\n{}", error_message, statement, self.underline_error(&error_message, error_col, error_length));
                    }
                    if operator_stack.is_empty() {
                        // Handle mismatched parentheses: no corresponding opening parenthesis
                        let error_col = token.position.col;
                        let error_length = token.length;
                        let error_message = format!("Unmatched ')' at row {}, column {}", token.position.row, error_col);
                        panic!("{}\n{}\n{}", error_message, statement, self.underline_error(&error_message, error_col, error_length));
                    }
                    while let Some(operator) = operator_stack.pop() {
                        if operator == Operator::OpenParen {
                            break;
                        }
                        let right = number_queue.pop().expect("Invalid equation: Missing operand");
                        let left = number_queue.pop().expect("Invalid equation: Missing operand");
                        let binary_expr = BinaryExpression {
                            left: Box::new(left),
                            right: Box::new(right),
                            operator,
                        };
                        number_queue.push(Node::BinaryExpression(binary_expr));
                    }
                    expect_operand = false;
                }
                _ => {
                    // Handle invalid token
                    let error_col = token.position.col;
                    let error_length = token.length;
                    let error_message = format!("Invalid token '{}' at row {}, column {}", token.text, token.position.row, error_col);
                    panic!("{}\n{}\n{}", error_message, statement, self.underline_error(&error_message, error_col, error_length));
                }
            }
        }
    
        if expect_operand {
            // Handle unexpected end of expression
            let error_col = tokens.last().unwrap().position.col;
            let error_length = tokens.last().unwrap().length;
            let error_message = format!("Unexpected end of expression at row {}, column {}", tokens.last().unwrap().position.row, error_col);
            panic!("{}\n{}\n{}", error_message, statement, self.underline_error(&error_message, error_col, error_length));
        }
    
        // Check for mismatched parentheses after parsing the equation
        if paren_count != 0 {
            let error_col = tokens.last().unwrap().position.col;
            let error_length = tokens.last().unwrap().length;
            let error_message = format!("Mismatched parentheses: Unclosed parenthesis at row {}, column {}", tokens.last().unwrap().position.row, error_col);
            panic!("{}\n{}\n{}", error_message, statement, self.underline_error(&error_message, error_col, error_length));
        }
    
        while let Some(operator) = operator_stack.pop() {
            // Pop remaining operators from the stack and construct binary expressions
            let right = number_queue.pop().unwrap();
            let left = number_queue.pop().unwrap();
    
            let binary_expr = BinaryExpression {
                left: Box::new(left),
                right: Box::new(right),
                operator,
            };
    
            number_queue.push(Node::BinaryExpression(binary_expr));
        }
    
        number_queue.pop().expect("No top node in operand stack")
    }
    
    fn handle_assignment(&self, tokens: &[Token]) -> VariableDeclaration {
        let current_line = tokens[0].position.row;
        let statement = self.get_nth_line("input.txt", current_line);
    
        self.validate_token(tokens.get(1).cloned(), 
                            TokenKind::Symbol,
                            None, 
                            "Expected identifier in assignment statement",
                            &statement);
    
        self.validate_token(tokens.get(2).cloned(), 
                            TokenKind::Operator, 
                            Some("="), 
                            "Expected '=' in assignment statement instead",
                            &statement);
    
        let eq = self.handle_equation(&tokens[3..tokens.len() - 1]);
        

        let id = Identifier { name: tokens.get(1).unwrap().text.clone() };
        let init = match eq {
            Node::BinaryExpression(binary_expr) => {
                InitExpression::BinaryExpression(binary_expr)
            },
            Node::Literal(literal) => {
                InitExpression::Literal(literal)
            },
            _ => panic!("Expected BinaryExpression or Literal"),
        };
        
        
        VariableDeclaration { id, init }
    }
    
    fn handle_if(&self) -> Node {
        todo!()
    }

    fn handle_while(&self) -> Node {
        todo!()
    }

    fn handle_function_assignment(&self) -> Node {
        todo!()
    }

    fn handle_keyword(&self, tokens: &[Token]) -> Node {
        match tokens[0].text.as_str() {
            "let" => Node::VariableDeclaration(self.handle_assignment(tokens)),
            "if" => self.handle_if(),
            "while" => self.handle_while(),
            "define" => self.handle_function_assignment(),
            _ => unreachable!()
        }
    }


    pub fn parse(&mut self) -> Program {
        let mut program = Program { program: Vec::new() };
        
        while self.position < self.length - 1 {
            let start = self.position;


            match self.tokens[self.position].kind {
                TokenKind::Keyword => {
                    while self.tokens[self.position].kind != TokenKind::LineBreak {
                        self.advance(1)
                    }
                    let res = self.handle_keyword(&self.tokens[start..=self.position]);
                    self.advance(1);
                    program.program.push(res);
                },
                TokenKind::Number => {
                    while self.tokens[self.position].kind != TokenKind::LineBreak {
                        self.advance(1)
                    }
                    let res = self.handle_equation(&self.tokens[start..self.tokens.len() - 1]);
                    self.advance(1);
                    program.program.push(res);
                },
                TokenKind::EOF => todo!(),
                _ => todo!(),
            } 
        }
        program
    }

    pub fn new(tokens: Vec<Token>) -> Parser {
        let length = tokens.len() as usize;
        Parser {
            tokens,
            length,
            position : 0,
        }
    }
}

