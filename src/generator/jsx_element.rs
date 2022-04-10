use crate::ast;

use super::context;
use super::{jsx_element_attribute, literal_string};

pub fn generate(ctx: &mut context::Context, ast: &ast::JsxElement) -> String {
    let name = ast.name.clone();
    let attrs = generate_attrs(ctx, &ast.attrs);

    if !ast.self_closing {
        let children = generate_children(ctx, &ast.children);
        format!("<{name}{attrs}>{children}</{name}>")
    } else {
        format!("<{name}{attrs} />")
    }
}

fn generate_children(ctx: &mut context::Context, children: &Vec<ast::Expr>) -> String {
    let children = children
        .iter()
        .map(|child| match child {
            ast::Expr::JsxElement(v) => generate(ctx, v),
            ast::Expr::LitString(v) => literal_string::generate(ctx, v, true, false),
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

fn generate_attrs(ctx: &mut context::Context, attrs: &Vec<ast::JsxElementAttribute>) -> String {
    let attrs = attrs
        .iter()
        .map(|attr| jsx_element_attribute::generate(ctx, attr))
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
