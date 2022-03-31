use crate::{ast::AST, scope::Scope};

use super::{checker_context::CheckerContext, statement};

pub fn check(context: &CheckerContext, scope: &mut Scope, if_statement: &AST) {
    if let AST::IfStatement {
        test: _,
        consequence,
        alternative,
        span: _,
    } = if_statement
    {
        check_consequence(context, scope, if_statement, consequence);
        check_alternative(context, scope, if_statement, alternative);
    }
}

pub fn check_consequence(
    context: &CheckerContext,
    scope: &mut Scope,
    if_statement: &AST,
    consequence: &Vec<AST>,
) {
    let mut consequence = consequence.iter();

    while let Some(consequence) = consequence.next() {
        statement::check(context, scope, consequence)
    }
}

pub fn check_alternative(
    context: &CheckerContext,
    scope: &mut Scope,
    if_statement: &AST,
    alternative: &Vec<AST>,
) {
    let mut alternative = alternative.iter();

    while let Some(alternative) = alternative.next() {
        statement::check(context, scope, alternative)
    }
}
