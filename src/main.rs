use std::fs;

mod error_message;
mod lexer;
mod parser;
mod token;

fn main() {
    let content = String::from_utf8_lossy(&fs::read("samples/functions.zac").unwrap()).to_string();
    let tokens = lexer::lex(content);
    let ast = parser::parse(tokens);
}
