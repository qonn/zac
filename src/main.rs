use std::{
    fs::{self, DirEntry},
    io,
    path::Path,
};

pub mod ast;
pub mod checker;
pub mod error_message;
mod generator;
pub mod lexer;
mod parser;
pub mod scope;
pub mod token;

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn main() {
    println!("");
    println!("parsing...");

    let _ = visit_dirs(Path::new("./samples"), &|d| {
        let filepath = d.path().to_string_lossy().to_string();
        println!("parsing {}", filepath);
        let content = String::from_utf8_lossy(&fs::read(d.path()).unwrap()).to_string();
        let mut lexer = lexer::new(&filepath, &content);
        let ast = parser::parse(&mut lexer);
        checker::check(&filepath, &content, &ast);
        generator::generate(filepath, &ast)
    });

    println!("successfully parse everything!");
}
