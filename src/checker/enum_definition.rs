use crate::{ast::AST, scope::Scope, token::Span};

use super::{context::CheckingContext, identifier};

pub fn check(ctx: &CheckingContext, scope: &mut Scope, ast: &AST) {
    if let AST::EnumDefinition {
        name,
        generics,
        items,
        span,
    } = ast
    {
        let scope = &mut scope.clone();
        define_generics_in_scope(generics, scope);
        check_name(ctx, span, scope, name);
        check_items(ctx, items, scope);
    }
}

fn define_generics_in_scope(generics: &Vec<AST>, scope: &mut Scope) {
    let mut generics_iter = generics.iter();

    while let Some(ast) = generics_iter.next() {
        if let AST::Identifier {
            value,
            generics: _,
            span: _,
        } = ast
        {
            scope.add_type_definition(value, ast);
        }
    }
}

pub fn check_name(ctx: &CheckingContext, span: &Span, scope: &mut Scope, name: &String) {
    if let Some(other) = scope.find_definition(name) {
        let message = format!("This enum name '{}' has been previously defined.", name);
        let pos = span.from;
        ctx.print_error_message(message, pos);

        let message = format!("It was previously defined here.");
        let pos = other.span().from;
        ctx.print_error_message(message, pos);

        panic!();
    }
}

pub fn check_items(ctx: &CheckingContext, items: &Vec<AST>, scope: &mut Scope) {
    let mut items_iter = items.iter();

    while let Some(item) = items_iter.next() {
        check_item(ctx, scope, item);
    }
}

pub fn check_item(ctx: &CheckingContext, scope: &mut Scope, item: &AST) {
    match item {
        AST::Identifier {
            value: _,
            generics,
            span: _,
        } => {
            identifier::check_generics(ctx, scope, generics);
        }
        AST::FunctionCall {
            callee: _,
            args,
            span: _,
        } => {
            identifier::check_multi(ctx, scope, args);
        }
        _ => {
            let message = format!(
                "Unexpected enum's item type '{:?}'.",
                crate::ast::ASTKind::from(item)
            );
            let pos = item.span().from;
            ctx.print_error_message(message, pos);
        }
    }
}
