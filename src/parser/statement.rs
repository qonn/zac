use crate::ast::AST;
use crate::token::TokenKind;

use super::context::ParsingContext;
use super::{identifier, js_literal, jsx_element, number_literal, string_literal};

pub fn parse(ctx: &mut ParsingContext) -> Option<AST> {
    let token = ctx.get_curr_token();

    match TokenKind::from(&token) {
        TokenKind::Js => js_literal::parse(ctx),
        TokenKind::Id => identifier::parse(ctx),
        TokenKind::Str => string_literal::parse(ctx),
        TokenKind::Numeric => number_literal::parse(ctx),
        TokenKind::JsxOpen => jsx_element::parse(ctx),
        TokenKind::NewLine => {
            ctx.eat(TokenKind::NewLine);
            None
        }
        _ => {
            ctx.throw_unexpected_token();
            None
        }
    }
}
