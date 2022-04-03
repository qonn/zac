use crate::ast::AST;
use crate::token::TokenKind;

use super::context::ParsingContext;
use super::{
    identifier, js_literal, jsx_element, number_literal, return_statement, string_literal,
    variable_statement,
};

pub fn parse(ctx: &mut ParsingContext) -> Option<AST> {
    let token = ctx.get_curr_token();

    match TokenKind::from(&token) {
        TokenKind::Js => js_literal::parse(ctx),
        TokenKind::Id => Some(identifier::parse(ctx)),
        TokenKind::Str => Some(string_literal::parse(ctx)),
        TokenKind::Numeric => Some(number_literal::parse(ctx)),
        TokenKind::JsxOpen => jsx_element::parse(ctx, false),
        TokenKind::Let => variable_statement::parse(ctx),
        TokenKind::Return => return_statement::parse(ctx),
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
