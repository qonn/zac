use crate::{ast::AST, scope::Scope, token::SourceSpan};

use super::{checker_context::CheckerContext, identifier};

pub fn check(context: &CheckerContext, scope: &mut Scope, ast: &AST) {
    if let AST::EnumDefinition {
        name,
        generics,
        items,
        span,
    } = ast
    {
        let scope = &mut scope.clone();
        define_generics_in_scope(generics, scope);
        check_name(context, span, scope, name);
        check_items(context, items, scope);
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

pub fn check_name(context: &CheckerContext, span: &SourceSpan, scope: &mut Scope, name: &String) {
    if let Some(other) = scope.find_definition(name) {
        let message = format!("This enum name '{}' has been previously defined.", name);
        let pos = span.from;
        context.print_error_message(message, pos);

        let message = format!("It was previously defined here.");
        let pos = other.source_span().from;
        context.print_error_message(message, pos);

        panic!();
    }
}

pub fn check_items(context: &CheckerContext, items: &Vec<AST>, scope: &mut Scope) {
    let mut items_iter = items.iter();

    while let Some(item) = items_iter.next() {
        check_item(context, scope, item);
    }
}

pub fn check_item(context: &CheckerContext, scope: &mut Scope, item: &AST) {
    match item {
        AST::Identifier {
            value: _,
            generics,
            span: _,
        } => {
            identifier::check_generics(context, scope, generics);
        }
        AST::FunctionCall {
            callee: _,
            args,
            span: _,
        } => {
            identifier::check_multi(context, scope, args);
        }
        _ => {
            let message = format!(
                "Unexpected enum's item type '{:?}'.",
                crate::ast::ASTKind::from(item)
            );
            let pos = item.source_span().from;
            context.print_error_message(message, pos);
        }
    }
}
