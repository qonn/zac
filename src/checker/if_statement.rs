use crate::{ast::AST, scope::Scope};

use super::{checker_context::CheckingContext, statement};

pub fn check(ctx: &mut CheckingContext, scope: &mut Scope, if_statement: &AST) {
    if let AST::IfStatement {
        test: _,
        consequence,
        alternative,
        span: _,
    } = if_statement
    {
        check_consequence(ctx, scope, if_statement, consequence);
        check_alternative(ctx, scope, if_statement, alternative);
    }
}

pub fn check_consequence(
    ctx: &mut CheckingContext,
    scope: &mut Scope,
    if_statement: &AST,
    consequence: &Vec<AST>,
) {
    let mut consequence = consequence.iter();

    while let Some(consequence) = consequence.next() {
        statement::check(ctx, scope, consequence)
    }
}

pub fn check_alternative(
    ctx: &mut CheckingContext,
    scope: &mut Scope,
    if_statement: &AST,
    alternative: &Vec<AST>,
) {
    let mut alternative = alternative.iter();

    while let Some(alternative) = alternative.next() {
        statement::check(ctx, scope, alternative)
    }
}
