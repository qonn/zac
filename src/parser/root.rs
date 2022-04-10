use crate::{ast, span::Span};

use super::{context::ParsingContext, statement};

pub fn parse(ctx: &mut ParsingContext) -> ast::Root {
    let mut stmts = vec![];

    while ctx.is_not_eof() {
        stmts.push(statement::parse(ctx));
    }

    ast::Root {
        name: ctx.lexer.filepath.clone(),
        path: ctx.lexer.filepath.clone(),
        stmts,
        span: Span::new(0, ctx.lexer.len),
    }
}
