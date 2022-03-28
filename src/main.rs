use std::{
    fs::{self, DirEntry},
    io,
    path::Path,
};

mod ast;
mod error_message;
mod lexer;
mod parser;
mod scope;
mod token;

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
        println!("parsing {}", d.path().to_string_lossy());
        let content = String::from_utf8_lossy(&fs::read(d.path()).unwrap()).to_string();
        let mut lexer = lexer::new(content);
        let mut parser = parser::parse(&mut lexer);
        let ast = parser.parse();
    });

    println!("successfully parse everything!");
}
