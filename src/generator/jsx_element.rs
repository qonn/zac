use crate::ast::{ASTKind, AST};

use super::{jsx_element_attribute, string_literal};

pub fn generate(ast: &AST) -> String {
    if let AST::JsxElement {
        name,
        attrs,
        children,
        self_closing,
        span,
    } = ast
    {
        let attrs = generate_attrs(attrs);

        if !self_closing {
            let children = generate_children(children);
            format!("<{name}{attrs}>{children}</{name}>")
        } else {
            format!("<{name}{attrs} />")
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

fn generate_attrs(attrs: &[AST]) -> String {
    let attrs = attrs
        .iter()
        .map(|attr| match ASTKind::from(attr) {
            ASTKind::JsxElementAttribute => jsx_element_attribute::generate(attr),
            _ => panic!("jsx_element.generate_attrs: Unexpected child AST Node {attr:#?}"),
        })
        .collect::<Vec<String>>()
        .join(" ")
        .trim()
        .to_string();

    if attrs.len() > 0 {
        format!(" {attrs}")
    } else {
        attrs
    }
}
