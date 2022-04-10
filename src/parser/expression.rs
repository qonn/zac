use crate::ast;
use crate::token::TokenKind;

use super::context::ParsingContext;
use super::{
    array, function, function_call, identifier, jsx_element, literal_boolean, literal_js,
    literal_number, literal_string, member_access,
};

pub fn parse(ctx: &mut ParsingContext) -> ast::Expr {
    ctx.eat_all_newlines();

    let mut expr = match TokenKind::from(ctx.get_curr_token()) {
        TokenKind::Str => literal_string::parse(ctx).into(),
        TokenKind::Boolean => literal_boolean::parse(ctx).into(),
        TokenKind::Numeric => literal_number::parse(ctx).into(),
        TokenKind::Js => literal_js::parse(ctx).into(),
        TokenKind::LParen => function::parse(ctx, true).into(),
        TokenKind::JsxOpen => jsx_element::parse(ctx, false).into(),
        TokenKind::Id => {
            let id = identifier::parse(ctx);

            let expr = match ctx.get_curr_token().kind() {
                TokenKind::LParen => function_call::parse(ctx, id).into(),
                _ => id.into(),
            };

            ctx.eat_all_newlines();

            expr
        }
        TokenKind::LSqrBr => ast::Expr::InitArray(array::parse(ctx)),
        _ => {
            ctx.throw_unexpected_token();
            panic!()
        }
    };

    if ctx.get_curr_token().kind() == TokenKind::Dot {
        expr = member_access::parse(ctx, expr).into();
    }

    ctx.eat_all_newlines();

    expr
}
