use crate::ast::AST;

use super::{context::ParsingContext, statement};

pub fn parse(ctx: &mut ParsingContext) -> AST {
    let mut children = vec![];

    while ctx.is_not_eof() {
        if let Some(child) = statement::parse(ctx) {
            children.push(child);
        }
    }

    AST::Root { children }
}
