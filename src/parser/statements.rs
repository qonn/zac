use super::{context::ParsingContext, statement};
use crate::{ast, token::TokenKind};

pub fn parse(ctx: &mut ParsingContext) -> Vec<ast::Stmt> {
    let mut stmts = vec![];

    while ctx.is_not_eof() && ctx.get_curr_token().kind() != TokenKind::RBrace {
        stmts.push(statement::parse(ctx));
    }

    stmts
}
