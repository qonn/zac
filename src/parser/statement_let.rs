use crate::ast;
use crate::span::Span;
use crate::token::TokenKind;

use super::context::ParsingContext;
use super::expression;

pub fn parse(ctx: &mut ParsingContext) -> ast::Let {
    let span_from = ctx.get_curr_token().span().from;

    ctx.eat(TokenKind::Let);

    let id = ctx.get_curr_token().value();

    ctx.eat(TokenKind::Id);
    ctx.eat(TokenKind::Eq);
    ctx.eat_all_newlines();

    let expr = expression::parse(ctx);

    let span_to = ctx.get_curr_token().span().from;

    ast::Let {
        id,
        expr,
        span: Span::new(span_from, span_to),
    }
}
