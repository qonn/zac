use crate::{
    ast::{ASTKind, AST},
    scope::Scope,
};

use super::{
    binary_expression, checker_context::CheckerContext, function_call, function_definition,
    identifier, literal,
};

pub fn check(context: &CheckerContext, scope: &mut Scope, ast: &AST) {
    match ast {
        AST::ArrayDeclarator { items, span: _ } => {
            check_items(context, scope, items);
        }
        _ => {
            let message = format!("Invalid AST Node '{:?}'", crate::ast::ASTKind::from(ast));
            let pos = ast.source_span().from;
            context.print_error_message(message, pos);
        }
    }
}

pub fn check_items(context: &CheckerContext, scope: &mut Scope, items: &Vec<AST>) {
    let mut items = items.iter();

    while let Some(item) = items.next() {
        check_item(context, scope, item);
    }
}

pub fn check_item(context: &CheckerContext, scope: &mut Scope, item: &AST) {
    match ASTKind::from(item) {
        ASTKind::Identifier => {
            identifier::check(context, scope, item);
        }
        ASTKind::StringLiteral => literal::check(context, scope, item),
        ASTKind::NumberLiteral => literal::check(context, scope, item),
        ASTKind::JsLiteral => literal::check(context, scope, item),
        ASTKind::FunctionCall => function_call::check(context, scope, item),
        ASTKind::BinaryExpression => binary_expression::check(context, scope, item),
        _ => {
            println!("{:?}", item);
            let message = format!("Unsupported array declaration syntax.");
            let pos = item.source_span().from;
            context.print_error_message(message, pos);
        }
    }
}
