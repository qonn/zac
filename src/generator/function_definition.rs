use crate::ast::{ASTKind, AST};

use super::{js_literal, jsx_element, return_statement, variable_statement};

pub fn generate(ast: &AST) -> String {
    if let AST::FunctionDefinition {
        name,
        args,
        expected_return_type: _,
        body,
        span: _,
    } = ast
    {
        format!(
            "function {}({}) {{{}}}",
            name,
            generate_args(args),
            generate_body(body)
        )
    } else {
        "".into()
    }
}

pub fn generate_args(args: &Vec<AST>) -> String {
    args.iter()
        .map(|arg| match ASTKind::from(arg) {
            ASTKind::FunctionArgumentDefinition => {
                if let AST::FunctionArgumentDefinition {
                    name,
                    type_: _,
                    span: _,
                } = arg
                {
                    name.to_string()
                } else {
                    "".into()
                }
            }
            _ => panic!("Unexpected arg AST Node {arg:#?}"),
        })
        .collect::<Vec<String>>()
        .join(", ")
}

pub fn generate_body(body: &Vec<AST>) -> String {
    let body = body
        .iter()
        .map(|child| match ASTKind::from(child) {
            ASTKind::JsLiteral => js_literal::generate(child),
            ASTKind::JsxElement => jsx_element::generate(child),
            ASTKind::ReturnStatement => return_statement::generate(child),
            ASTKind::VariableStatement => variable_statement::generate(child),
            _ => panic!("Unexpected child AST Node {child:#?}"),
        })
        .collect::<Vec<String>>()
        .join("\n")
        .split("\n")
        .map(|x| format!("  {x}"))
        .collect::<Vec<String>>()
        .join("\n");

    let body = format!("\n{body}\n");

    body
}
