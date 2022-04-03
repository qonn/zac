use crate::{
    ast::AST,
    token::{SourceSpan, TokenKind},
};

use super::{context::ParsingContext, expression};

pub fn parse(ctx: &mut ParsingContext) -> Option<AST> {
    let span_from = ctx.get_curr_token().span().from;

    ctx.eat(TokenKind::Return);
    ctx.eat_all_newlines();

    let expr = if let Some(ast) = expression::parse(ctx) {
        Some(Box::new(ast))
    } else {
        None
    };

    let span_to = ctx.get_prev_token().span().from;

    Some(AST::ReturnStatement {
        expr,
        span: SourceSpan::new(span_from, span_to),
    })
}
