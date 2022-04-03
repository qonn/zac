use crate::ast::AST;
use crate::token::TokenKind;

use super::context::ParsingContext;

pub fn parse(ctx: &mut ParsingContext) -> AST {
    let value = ctx.get_curr_token().value();
    let span = ctx.get_curr_token().span();
    ctx.eat(TokenKind::Numeric);
    AST::NumberLiteral { value, span }
}
