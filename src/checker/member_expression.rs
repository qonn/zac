use crate::{ast::AST, scope::Scope, token::SourceSpan};

use super::{checker_context::CheckerContext, identifier};

pub fn check(context: &CheckerContext, scope: &mut Scope, ast: &AST) {
    if let AST::MemberExpression {
        object,
        property,
        span,
    } = ast
    {
        check_object(context, scope, ast, object);
        check_property(context, scope, ast, property);
    }
}

pub fn check_object(context: &CheckerContext, scope: &mut Scope, parent: &AST, object: &AST) {}

pub fn check_property(context: &CheckerContext, scope: &mut Scope, parent: &AST, property: &AST) {
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
