use crate::ast::AST;
use crate::token::TokenKind;

use super::context::ParsingContext;
use super::{
    function_anon_definition, identifier, js_literal, jsx_element, number_literal, string_literal,
};

pub fn parse(ctx: &mut ParsingContext) -> Option<AST> {
    match TokenKind::from(ctx.get_curr_token()) {
        TokenKind::Str => Some(string_literal::parse(ctx)),
        TokenKind::Numeric => Some(number_literal::parse(ctx)),
        TokenKind::Js => js_literal::parse(ctx),
        TokenKind::LParen => function_anon_definition::parse(ctx),
        TokenKind::JsxOpen => jsx_element::parse(ctx, false),
        TokenKind::Id => Some(identifier::parse(ctx)),
        _ => {
            ctx.throw_unexpected_token();
            None
        }
    }
}
