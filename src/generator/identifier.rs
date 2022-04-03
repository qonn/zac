use crate::ast::{ASTKind, AST};

use super::{js_literal, jsx_element, string_literal};

pub fn generate(ast: &AST) -> String {
    match ast {
        AST::Identifier {
            value,
            generics: _,
            span: _,
        } => value.to_string(),
        _ => panic!("identifier.generate: Unexpected AST Node {ast:#?}"),
    }
}
