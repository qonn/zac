use crate::ast::AST;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref VARIABLE_REPLACEMENT: Regex = Regex::new(r#"#\{(.*)\}"#).unwrap();
}

pub fn generate(ast: &AST, jsx: bool, jsx_attr: bool) -> String {
    if let AST::StringLiteral { value, span: _ } = ast {
        let mut final_value = value.clone();

        for found in VARIABLE_REPLACEMENT.find_iter(value) {
            let to_replace = found.as_str();
            if let Some(caps) = VARIABLE_REPLACEMENT.captures(to_replace) {
                let variable = &format!("${{{}}}", &caps[1]);
                final_value = final_value.replace(to_replace, variable);
            }
        }

        if jsx {
            format!("{}", final_value.to_string())
        } else if jsx_attr {
            format!("\"{}\"", final_value.to_string())
        } else {
            format!("`{}`", final_value.to_string())
        }
    } else {
        "".into()
    }
}
