use crate::{ast::AST, scope::Scope, token::SourceSpan};

use super::{checker_context::CheckingContext, identifier};

pub fn check(ctx: &CheckingContext, scope: &mut Scope, ast: &AST) {
    if let AST::RecordDefinition { keys, name, span } = ast {
        check_name(ctx, span, scope, name);
        check_keys(ctx, keys, scope);
    }
}

pub fn check_name(ctx: &CheckingContext, span: &SourceSpan, scope: &mut Scope, name: &String) {
    if let Some(other) = scope.get_record_definition(name) {
        let message = format!("This record name '{}' has been previously defined.", name);
        let pos = span.from;
        ctx.print_error_message(message, pos);

        let message = format!("It was previously defined here.");
        let pos = other.source_span().from;
        ctx.print_error_message(message, pos);

        panic!();
    }
}

pub fn check_keys(ctx: &CheckingContext, keys: &Vec<AST>, scope: &mut Scope) {
    let mut keys_iter = keys.iter();

    while let Some(key) = keys_iter.next() {
        if let AST::RecordKeyDefinition { name, kind, span } = key {
            if let Some(kind) = kind.first() {
                identifier::check(ctx, scope, kind);
            } else {
                let message = format!("Invalid AST Node '{:?}'", crate::ast::ASTKind::from(key));
                let pos = key.source_span().from;
                ctx.print_error_message(message, pos);
            }
        } else {
            let message = format!("Invalid AST Node '{:?}'", crate::ast::ASTKind::from(key));
            let pos = key.source_span().from;
            ctx.print_error_message(message, pos);
        }
    }
}
