use crate::ast;

use super::context;
use super::expression;

pub fn generate(ctx: &mut context::Context, v: &ast::Binary) -> String {
    let op = v.op.value();

    let left = expression::generate(ctx, &v.left);
    let right = expression::generate(ctx, &v.right);
    return format!("{left} {op} {right}");
}
