use crate::{ast, token::TokenKind};

use super::context::ParsingContext;

pub fn parse(ctx: &mut ParsingContext) -> ast::LitNumber {
    let raw = ctx.get_curr_token().value();
    let value = ctx.get_curr_token().value().parse::<f64>().unwrap();
    let span = ctx.get_curr_token().span();
    ctx.eat(TokenKind::Numeric);
    ast::LitNumber { raw, value, span }
}
