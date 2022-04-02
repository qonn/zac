use crate::ast::{ASTKind, AST};

use super::{function_definition, js_literal};

pub fn generate(ast: &AST) -> String {
    if let AST::FunctionCall {
        callee,
        args,
        span: _,
    } = ast
    {
        format!("{}({})", generate_callee(callee), generate_args(args))
    } else {
        "".into()
    }
}

pub fn generate_callee(callee: &AST) -> String {
    match callee {
        AST::Identifier {
            value,
            generics: _,
            span: _,
        } => value.to_string(),
        _ => panic!("Unexpected callee AST Node {callee:#?}"),
    }
}

pub fn generate_args(args: &Vec<AST>) -> String {
    args.iter()
        .map(|arg| match ASTKind::from(arg) {
            ASTKind::FunctionDefinition => function_definition::generate(arg),
            ASTKind::JsLiteral => js_literal::generate(arg),
            _ => panic!("Unexpected arg AST Node {arg:#?}"),
        })
        .collect::<Vec<String>>()
        .join(", ")
}
