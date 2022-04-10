use lazy_static::lazy_static;
use regex::Regex;

use crate::ast;

lazy_static! {
    static ref VARIABLE_REPLACEMENT: Regex = Regex::new(r#"#\{(.*)\}"#).unwrap();
}

pub fn generate(ast: &ast::LitNumber) -> String {
    format!("{}", ast.value.to_string())
}
