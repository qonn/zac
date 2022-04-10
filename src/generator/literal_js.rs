use lazy_static::lazy_static;
use regex::Regex;

use super::context;
use crate::ast;

lazy_static! {
    static ref VARIABLE_REPLACEMENT: Regex = Regex::new(r#"#\{([a-zA-Z0-9_]*)\}"#).unwrap();
}

pub fn generate(ctx: &mut context::Context, ast: &ast::LitJs) -> String {
    let mut final_value = ast.value.clone();

    for found in VARIABLE_REPLACEMENT.find_iter(&ast.value) {
        let to_replace = found.as_str();
        if let Some(caps) = VARIABLE_REPLACEMENT.captures(to_replace) {
            let variable = &caps[1];
            final_value = final_value.replace(to_replace, variable);
        }
    }

    final_value.to_string()
}
