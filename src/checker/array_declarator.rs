use crate::{
    ast::{ASTKind, AST},
    scope::Scope,
};

use super::{
    binary_expression, context::CheckingContext, function_call, identifier, literal,
    type_resolver,
};

pub fn check(ctx: &mut CheckingContext, scope: &mut Scope, ast: &AST) {
    match ast {
        AST::ArrayDeclarator { items, span: _ } => {
            check_items(ctx, scope, ast, items);
        }
        _ => {
            let message = format!("Invalid AST Node '{:?}'", crate::ast::ASTKind::from(ast));
            let pos = ast.source_span().from;
            ctx.print_error_message(message, pos);
        }
    }
}

pub fn check_items(
    ctx: &mut CheckingContext,
    scope: &mut Scope,
    array_declarator: &AST,
    array_declarator_items: &Vec<AST>,
) {
    let mut items = array_declarator_items.iter();
    let mut last_item_type = "".to_string();

    while let Some(array_declarator_item) = items.next() {
        check_item(ctx, scope, array_declarator_item);
        let resolved_item_type =
            resolve_item_type(ctx, scope, array_declarator, array_declarator_item);
        if last_item_type != "" && resolved_item_type != last_item_type {
            let message = format!(
                "Invalid array item's type, was expecting '{}' but this item type is '{}'",
                last_item_type, resolved_item_type
            );
            let pos = array_declarator_item.source_span().from;
            ctx.print_error_message(message, pos);
        } else {
            last_item_type = resolved_item_type;
        }
    }
}

pub fn check_item(ctx: &mut CheckingContext, scope: &mut Scope, item: &AST) {
    match ASTKind::from(item) {
        ASTKind::Identifier => {
            identifier::check(ctx, scope, item);
        }
        ASTKind::StringLiteral => literal::check(ctx, scope, item),
        ASTKind::NumberLiteral => literal::check(ctx, scope, item),
        ASTKind::JsLiteral => literal::check(ctx, scope, item),
        ASTKind::FunctionCall => function_call::check(ctx, scope, item),
        ASTKind::BinaryExpression => binary_expression::check(ctx, scope, item),
        _ => {
            let message = format!("Unsupported array declaration syntax.");
            let pos = item.source_span().from;
            ctx.print_error_message(message, pos);
        }
    }
}

pub fn resolve_item_type(
    ctx: &mut CheckingContext,
    scope: &mut Scope,
    array_declarator: &AST,
    array_declarator_item: &AST,
) -> String {
    type_resolver::resolve(ctx, scope, array_declarator_item)
}
