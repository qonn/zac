use crate::{ast, span::Span, token::TokenKind};

use super::{context::ParsingContext, expression};

pub fn parse(ctx: &mut ParsingContext) -> ast::Return {
    let span_from = ctx.get_curr_token().span().from;

    ctx.eat(TokenKind::Return);
    ctx.eat_all_newlines();

    let expr = expression::parse(ctx);

    let span_to = ctx.get_prev_token().span().from;

    ast::Return {
        expr,
        span: Span::new(span_from, span_to),
    }
}
