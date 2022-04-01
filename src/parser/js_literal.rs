use crate::ast::AST;
use crate::token::TokenKind;

use super::context::ParsingContext;

pub fn parse(ctx: &mut ParsingContext) -> Option<AST> {
    let current_token = ctx.get_curr_token();
    ctx.eat(TokenKind::Js);
    Some(AST::JsLiteral {
        value: current_token.value(),
        span: current_token.span().clone(),
    })
}
