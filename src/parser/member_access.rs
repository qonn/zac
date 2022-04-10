use crate::{
    ast,
    span::{Span, Spanned},
    token::TokenKind,
};

use super::{context::ParsingContext, expression};

pub fn parse(ctx: &mut ParsingContext, obj: ast::Expr) -> ast::MemberAccess {
    ctx.eat(TokenKind::Dot);

    let prop = expression::parse(ctx);

    let span_from = obj.span().from;
    let span_to = prop.span().to;

    let expr = ast::MemberAccess {
        obj,
        prop,
        span: Span::new(span_from, span_to),
    };

    expr
}
