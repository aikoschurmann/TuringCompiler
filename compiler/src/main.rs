mod lexer;
mod lexer_types;


use lexer::Lexer;

use std::fs;


fn main() { 
    let code = fs::read_to_string("input.txt").unwrap();
    let mut lexer = Lexer::new(code.clone());

    let tokens = lexer.tokenize();

    let tokens_json: String = serde_json::to_string_pretty(&tokens).unwrap();
    fs::write("compiler-steps/step1.json", format!("{}", tokens_json)).unwrap();

}
