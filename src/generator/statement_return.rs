use crate::{ast, generator::expression};

use super::context;

pub fn generate(ctx: &mut context::Context, ast: &ast::Return) -> String {
    format!("return {}", expression::generate(ctx, &ast.expr))
}
