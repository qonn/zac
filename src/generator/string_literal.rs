use crate::ast::AST;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref VARIABLE_REPLACEMENT: Regex = Regex::new(r#"#\{([a-zA-Z0-9_]*)\}"#).unwrap();
}

pub fn generate(ast: &AST, jsx: bool) -> String {
    if let AST::StringLiteral { value, span: _ } = ast {
        let mut final_value = value.clone();

        for found in VARIABLE_REPLACEMENT.find_iter(value) {
            let to_replace = found.as_str();
            if let Some(caps) = VARIABLE_REPLACEMENT.captures(to_replace) {
                let variable = &caps[1];
                final_value = final_value.replace(to_replace, variable);
            }
        }

        if jsx {
            format!("{}", value.to_string())
        } else {
            format!("\"{}\"", value.to_string())
        }
    } else {
        "".into()
    }
}
