use crate::ast::{ASTKind, AST};

use super::{function_call, identifier, js_literal, jsx_element, number_literal, string_literal};

pub fn generate(ast: &AST) -> String {
    match ASTKind::from(ast) {
        ASTKind::JsLiteral => js_literal::generate(ast),
        ASTKind::JsxElement => jsx_element::generate(ast),
        ASTKind::StringLiteral => string_literal::generate(ast, false, false),
        ASTKind::Identifier => identifier::generate(ast),
        ASTKind::FunctionCall => function_call::generate(ast),
        ASTKind::NumberLiteral => number_literal::generate(ast),
        _ => panic!("expression.generate: Unexpected AST Node {ast:#?}"),
    }
}
