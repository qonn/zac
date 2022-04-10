use crate::{ast, token::TokenKind};

use super::context::ParsingContext;

pub fn parse(ctx: &mut ParsingContext) -> ast::LitBoolean {
    let raw = ctx.get_curr_token().value();
    let value = ctx.get_curr_token().value().parse::<bool>().unwrap();
    let span = ctx.get_curr_token().span();
    ctx.eat(TokenKind::Boolean);
    ast::LitBoolean { raw, value, span }
}
