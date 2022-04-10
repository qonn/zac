use crate::{ast, token::TokenKind};

use super::context::ParsingContext;

pub fn parse(ctx: &mut ParsingContext) -> ast::LitJs {
    let current_token = ctx.get_curr_token();
    let value = current_token.value();
    let span = current_token.span().clone();

    ctx.eat(TokenKind::Js);
    ast::LitJs {
        raw: format!("\"{value}\""),
        value,
        span,
    }
}
