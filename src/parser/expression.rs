use crate::ast::AST;
use crate::token::TokenKind;

use super::context::ParsingContext;
use super::{function_anon_definition, js_literal, number_literal, string_literal};

pub fn parse(ctx: &mut ParsingContext) -> Option<AST> {
    match TokenKind::from(ctx.get_curr_token()) {
        TokenKind::Str => string_literal::parse(ctx),
        TokenKind::Numeric => number_literal::parse(ctx),
        TokenKind::Js => js_literal::parse(ctx),
        TokenKind::LParen => function_anon_definition::parse(ctx),
        _ => None,
    }
}
