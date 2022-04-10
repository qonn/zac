use crate::{
    ast::{self, Ident},
    span::{Span, Spanned},
    token::{Token, TokenKind},
};

use super::{context::ParsingContext, expression};

pub fn parse(ctx: &mut ParsingContext, id: Ident) -> ast::FnCall {
    let span_from = id.span().from;

    ctx.eat(TokenKind::LParen);

    let args = parse_args(ctx);

    ctx.eat_all_newlines();

    let span_to = ctx.get_curr_token().span().to;

    ctx.eat(TokenKind::RParen);

    ast::FnCall {
        id,
        args,
        span: Span::new(span_from, span_to),
    }
}

fn parse_args(ctx: &mut ParsingContext) -> Vec<ast::Expr> {
    let mut args = vec![];

    while ctx.is_not_eof() {
        if let Token::RParen(_) = ctx.get_curr_token() {
            break;
        }

        ctx.eat_all_newlines();

        args.push(expression::parse(ctx));

        if let Token::Comma(_) = ctx.get_curr_token() {
            ctx.eat(TokenKind::Comma);
        }

        ctx.eat_all_newlines();
    }

    args
}
