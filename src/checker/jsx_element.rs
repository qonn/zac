use super::context::CheckingContext;
use crate::{ast::AST, scope::Scope};

pub fn check(ctx: &mut CheckingContext, scope: &mut Scope, ast: &AST) {
    if let AST::JsxElement { name, attrs, children, span } = ast {

    } else {

    }
}
