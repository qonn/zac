use crate::{ast::AST, scope::Scope, token::SourceSpan};

use super::{context::CheckingContext, identifier};

pub fn check(ctx: &CheckingContext, scope: &mut Scope, ast: &AST) {
    if let AST::MemberExpression {
        object,
        property,
        span,
    } = ast
    {
        check_object(ctx, scope, ast, object);
        check_property(ctx, scope, ast, property);
    }
}

pub fn check_object(ctx: &CheckingContext, scope: &mut Scope, parent: &AST, object: &AST) {}

pub fn check_property(ctx: &CheckingContext, scope: &mut Scope, parent: &AST, property: &AST) {
    if let AST::Identifier {
        value,
        generics: _,
        span: _,
    } = property
    {
        if value == "await" {
            return;
        }
    }
}
