mod lexer;
mod ast;

use std::fs;
use lexer::*;
use ast::*;

fn main() {
    let code = fs::read_to_string("input.txt").unwrap();
    let mut lexer = Lexer::new(code.clone());
    println!("{:?}", code);
    println!("");
    println!("Lexing!");

    let tokens = lexer.tokenize();


    for token in tokens.clone(){
        println!("{:?}", token)
    }
    println!("");

    println!("Building a tree!");

    let mut parser = Parser::new(tokens.clone());
    let _ast = parser.parse();
    let ast_json: String = serde_json::to_string(&_ast).unwrap();


    println!("{:?}", _ast)
}