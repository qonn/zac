use crate::ast::AST;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref VARIABLE_REPLACEMENT: Regex = Regex::new(r#"#\{(.*)\}"#).unwrap();
}

pub fn generate(ast: &AST) -> String {
    if let AST::NumberLiteral { value, span: _ } = ast {
        format!("{}", value.to_string())
    } else {
        "".into()
    }
}
