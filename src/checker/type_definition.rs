use crate::{ast::AST, scope::Scope, token::SourceSpan};

use super::{checker_context::CheckerContext, identifier};

pub fn check(context: &CheckerContext, scope: &mut Scope, ast: &AST) {
    if let AST::TypeDefinition {
        name,
        generics,
        items,
        span,
    } = ast
    {
        check_name(context, scope, name, span);

        let mut item_iters = items.iter();

        while let Some(ast) = item_iters.next() {
            check_item(context, scope, ast);
        }
    }
}

pub fn check_name(context: &CheckerContext, scope: &mut Scope, name: &String, span: &SourceSpan) {
    if let Some(other) = scope.get_type_definition(name) {
        let message = format!("The type '{}' has been previously defined.", name);
        let pos = span.from;
        context.print_error_message(message, pos);

        let message = format!("It was previously defined here.");
        let pos = other.source_span().from;
        context.print_error_message(message, pos);

        panic!();
    }
}

pub fn check_item(context: &CheckerContext, scope: &mut Scope, item: &AST) {
    match item {
        AST::Identifier {
            value: _,
            generics: _,
            span: _,
        } => {
            identifier::check(context, scope, item);
        }
        AST::FunctionCall {
            callee: _,
            args: _,
            span: _,
        } => check(context, scope, item),
        _ => {
            let message = format!("Unsupported type definition syntax.");
            let pos = item.source_span().from;
            context.print_error_message(message, pos);
        }
    }
}
