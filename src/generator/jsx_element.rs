use crate::ast::{ASTKind, AST};

use super::{function_definition, js_literal, string_literal};

pub fn generate(ast: &AST) -> String {
    if let AST::JsxElement {
        name,
        attrs,
        children,
        self_closing,
        span,
    } = ast
    {
        if !self_closing {
            let children = generate_children(children);
            format!("<{name}>{children}</{name}>")
        } else {
            format!("<{name} />")
        }
    } else {
        "".into()
    }
}

fn generate_children(children: &[AST]) -> String {
    let children = children
        .iter()
        .map(|child| match ASTKind::from(child) {
            ASTKind::JsxElement => generate(child),
            ASTKind::StringLiteral => string_literal::generate(child, true),
            _ => panic!("jsx_element: Unexpected child AST Node {child:#?}"),
        })
        .collect::<Vec<String>>()
        .join("\n")
        .split("\n")
        .map(|x| format!("  {x}"))
        .collect::<Vec<String>>()
        .join("\n");

    format!("\n{children}\n")
}
