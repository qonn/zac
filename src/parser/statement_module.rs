use super::{context::ParsingContext, statements};
use crate::{ast, span::Span, token::TokenKind};

pub fn parse(ctx: &mut ParsingContext) -> ast::Mod {
    let span_from = ctx.get_curr_token().span().from;

    ctx.eat(TokenKind::Mod);

    let mut paths = vec![];

    while ctx.get_curr_token().kind() == TokenKind::Id {
        paths.push(ctx.get_curr_token().value());

        ctx.eat(TokenKind::Id);

        if ctx.get_curr_token().kind() == TokenKind::Dot {
            ctx.eat(TokenKind::Dot);
        }

        if ctx.get_curr_token().kind() == TokenKind::LBrace {
            break;
        }
    }

    let path = paths.join(".");

    ctx.eat(TokenKind::LBrace);

    let stmts = statements::parse(ctx);

    let span_to = ctx.get_curr_token().span().to;

    ctx.eat(TokenKind::RBrace);

    ast::Mod {
        path,
        stmts,
        span: Span::new(span_from, span_to),
    }
}
