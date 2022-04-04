mod expression;
mod function_call;
mod function_definition;
mod identifier;
mod js_literal;
mod jsx_element;
mod jsx_element_attribute;
mod number_literal;
mod return_statement;
mod root;
mod string_literal;
mod variable_statement;

use std::{fs, path::Path};

use crate::{ast::AST, utils::normalize_path};

pub fn generate(filepath: String, root_ast: &AST) {
    let source_folder_path = Path::new(&filepath);
    let source_file_name = source_folder_path.file_name().unwrap().to_str().unwrap();
    let target_folder_path = &filepath.replace("samples", "build");
    let target_folder_path = normalize_path(&Path::new(target_folder_path).join(".."));
    let target_file_path = target_folder_path.join(source_file_name.replace(".zac", ".jsx"));

    let _ = fs::create_dir_all(&target_folder_path);
    let content = root::generate(root_ast);
    let _ = fs::write(target_file_path, content);
}
