mod binary;
mod context;
mod expression;
mod fn_call;
mod identifier;
mod init_array;
mod jsx_element;
mod jsx_element_attribute;
mod literal_js;
mod literal_number;
mod literal_string;
mod member_access;
mod root;
mod statement_fn;
mod statement_if;
mod statement_let;
mod statement_mod;
mod statement_return;
mod literal_boolean;

use std::{fs, path::Path};

use crate::{ast, utils::normalize_path};

pub fn generate(file_path: String, file_content: String, root_ast: &ast::Root) {
    let source_folder_path = Path::new(&file_path);
    let source_file_name = source_folder_path.file_name().unwrap().to_str().unwrap();
    let target_folder_path = &file_path.replace("samples", "build");
    let target_folder_path = normalize_path(&Path::new(target_folder_path).join(".."));
    let target_file_path = target_folder_path.join(source_file_name.replace(".zac", ".jsx"));

    let _ = fs::create_dir_all(&target_folder_path);
    let content = root::generate(
        &mut context::Context::new(file_path, file_content),
        root_ast,
    );
    let _ = fs::write(target_file_path, content);
}
