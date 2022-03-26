use std::fs;

mod ast;
mod error_message;
mod lexer;
mod parser;
mod scope;
mod token;

fn main() {
    println!("");
    println!("parsing...");
    let content = String::from_utf8_lossy(&fs::read("samples/variables.zac").unwrap()).to_string();
    let mut lexer = lexer::new(content);
    let mut parser = parser::parse(&mut lexer);
    let ast = parser.parse();
    println!("successfully parse!");
}
