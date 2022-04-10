use std::{
    fs::{self, DirEntry},
    io,
    path::Path,
    time::Instant,
};

pub mod ast;
pub mod error_message;
pub mod generator;
pub mod lexer;
mod parser;
mod span;
pub mod token;
mod utils;

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
    println!("compiling...");

    let start = Instant::now();
    let _ = visit_dirs(Path::new("./samples"), &|d| {
        let file_path = crate::utils::normalize_path(&d.path())
            .to_string_lossy()
            .to_string();

        // if !filepath.contains("router") {
        //     return;
        // }

        if !file_path.contains(".zac") {
        } else {
            println!("compiling {}", file_path);
            let file_content = String::from_utf8_lossy(&fs::read(d.path()).unwrap()).to_string();
            let mut lexer = lexer::new(&file_path, &file_content);
            let ast = parser::parse(&mut lexer);
            // println!("{ast:#?}");
            // checker::check(&filepath, &content, &ast);
            generator::generate(file_path, file_content, &ast)
        }
    });
    let duration = start.elapsed();
    println!("compilation done in {duration:?}");
}
