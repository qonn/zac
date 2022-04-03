use crate::{
    ast::{ASTKind, AST},
    generator::string_literal,
};

pub fn generate(ast: &AST) -> String {
    if let AST::JsxElementAttribute {
        name,
        expr,
        span: _,
    } = ast
    {
        let expr = match ASTKind::from(expr.as_ref()) {
            ASTKind::StringLiteral => string_literal::generate(expr, false),
            _ => panic!("jsx_element_attribute.generate: Unexpected child AST Node {expr:#?}"),
        };

        format!("{name}={expr}")
    } else {
        "".to_string()
    }
}
