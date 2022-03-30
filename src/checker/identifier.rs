use crate::{ast::AST, scope::Scope};

use super::checker_context::CheckerContext;

pub fn check(context: &CheckerContext, scope: &mut Scope, kind: &AST) {
    match kind {
        AST::Identifier {
            value,
            generics,
            span,
        } => {
            if !scope.is_defined(value) {
                let message = format!("The type '{:?}' used here could not be found.", value);
                let pos = span.from;
                context.print_error_message(message, pos);
                return;
            }

            check_generics(context, scope, generics);
        }
        _ => {
            let message = format!("Invalid AST Node '{:?}'", crate::ast::ASTKind::from(kind));
            let pos = kind.source_span().from;
            context.print_error_message(message, pos);
        }
    }
}

pub fn check_multi(context: &CheckerContext, scope: &mut Scope, multi: &Vec<AST>) {
    let mut multi_iter = multi.iter();

    while let Some(item) = multi_iter.next() {
        check(context, scope, item);
    }
}

pub fn check_generics(context: &CheckerContext, scope: &mut Scope, generics: &Vec<AST>) {
    let mut generics_iter = generics.iter();

    while let Some(generic) = generics_iter.next() {
        check(context, scope, generic);
    }
}
