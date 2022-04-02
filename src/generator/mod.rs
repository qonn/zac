mod root;
mod function_definition;
mod function_call;
mod js_literal;
mod jsx_element;
mod string_literal;

use std::{
    fs,
    path::{Component, Path, PathBuf},
};

use crate::ast::AST;

pub fn generate(filepath: String, root_ast: &AST) {
    let source_folder_path = Path::new(&filepath);
    let source_file_name = source_folder_path.file_name().unwrap();
    let source_folder_path = source_folder_path.join("..");
    let source_folder_path = normalize_path(&source_folder_path);

    let target_folder_path = &filepath.replace("samples", "build");
    let target_folder_path = normalize_path(&Path::new(target_folder_path).join(".."));

    println!(
        "{}, {}, {}, {}",
        source_file_name.to_str().unwrap(),
        source_folder_path.to_str().unwrap(),
        target_folder_path.to_str().unwrap(),
        target_folder_path.join(source_file_name).to_str().unwrap(),
    );

    let _ = fs::create_dir_all(&target_folder_path);
    let content = root::generate(root_ast);
    let _ = fs::write(target_folder_path.join(source_file_name), content);
}

pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}
