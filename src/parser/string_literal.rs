use crate::ast::AST;
use crate::token::TokenKind;

use super::context::ParsingContext;

pub fn parse(ctx: &mut ParsingContext) -> Option<AST> {
    let value = ctx.get_curr_token().value();
    let span = ctx.get_curr_token().span();
    ctx.eat(TokenKind::Str);
    Some(AST::StringLiteral { value, span })
}
