use crate::{ast::AST, scope::Scope};

use super::checker_context::CheckingContext;

pub fn check(ctx: &CheckingContext, scope: &mut Scope, kind: &AST) {
    match kind {
        AST::Identifier {
            value,
            generics,
            span,
        } => {
            if !scope.is_defined(value) {
                let message = format!("The type '{:?}' used here could not be found.", value);
                let pos = span.from;
                ctx.print_error_message(message, pos);
                return;
            }

            check_generics(ctx, scope, generics);
        }
        _ => {
            let message = format!("Invalid AST Node '{:?}'", crate::ast::ASTKind::from(kind));
            let pos = kind.source_span().from;
            ctx.print_error_message(message, pos);
        }
    }
}

pub fn check_multi(ctx: &CheckingContext, scope: &mut Scope, multi: &Vec<AST>) {
    let mut multi_iter = multi.iter();

    while let Some(item) = multi_iter.next() {
        check(ctx, scope, item);
    }
}

pub fn check_generics(ctx: &CheckingContext, scope: &mut Scope, generics: &Vec<AST>) {
    let mut generics_iter = generics.iter();

    while let Some(generic) = generics_iter.next() {
        check(ctx, scope, generic);
    }
}
