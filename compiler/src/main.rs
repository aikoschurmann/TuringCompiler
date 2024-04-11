mod ast;
mod lexer;

use lexer::Lexer;

use std::fs;

use crate::ast::Parser;


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
    let ast_json: String = serde_json::to_string_pretty(&_ast).unwrap();
    fs::write("compiler-steps/step2.json", format!("{}", ast_json)).unwrap();


    println!("{:?}", _ast)
}