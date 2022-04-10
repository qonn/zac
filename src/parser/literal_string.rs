use crate::{ast, token::TokenKind};

use super::context::ParsingContext;

pub fn parse(ctx: &mut ParsingContext) -> ast::LitString {
    let raw = format!("\"{}\"", ctx.get_curr_token().value());
    let value = ctx.get_curr_token().value();
    let span = ctx.get_curr_token().span();
    ctx.eat(TokenKind::Str);
    ast::LitString { raw, value, span }
}
