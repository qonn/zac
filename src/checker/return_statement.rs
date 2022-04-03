use super::context::CheckingContext;

use crate::{
    ast::{ASTKind, AST},
    scope::Scope,
};

pub fn check(ctx: &mut CheckingContext, scope: &Scope, ast: &AST) {
    if let Some(owner) = &scope.owner {
        if ASTKind::from(owner) == ASTKind::FunctionDefinition {}
    }
}
