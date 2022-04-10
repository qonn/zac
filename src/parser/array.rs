use super::context::ParsingContext;
use super::expression;
use crate::ast;
use crate::span::Span;
use crate::token::TokenKind;

pub fn parse(ctx: &mut ParsingContext) -> ast::InitArray {
    let span_from = ctx.get_curr_token().span().from;

    ctx.eat(TokenKind::LSqrBr);
    ctx.eat_all_newlines();

    let mut items = vec![];

    while ctx.is_not_eof() {
        if ctx.get_curr_token().kind() == TokenKind::RSqrBr {
            break;
        }

        items.push(parse_item(ctx));
    }

    ctx.eat(TokenKind::RSqrBr);

    let span_to = ctx.get_prev_token().span().to;

    ast::InitArray {
        items,
        span: Span::new(span_from, span_to),
    }
}

pub fn parse_item(ctx: &mut ParsingContext) -> ast::Expr {
    let ast = expression::parse(ctx);

    ctx.eat_all_newlines();

    if ctx.get_curr_token().kind() != TokenKind::RSqrBr {
        ctx.eat(TokenKind::Comma);
    }

    ctx.eat_all_newlines();

    return ast;
}
