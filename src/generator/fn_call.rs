use crate::ast;

use super::context;
use super::expression;

pub fn generate(ctx: &mut context::Context, ast: &ast::FnCall) -> String {
    format!(
        "{}({})",
        generate_callee(&ast.id),
        generate_args(ctx, &ast.args)
    )
}

pub fn generate_callee(id: &ast::Ident) -> String {
    id.string.to_string()
}

pub fn generate_args(ctx: &mut context::Context, args: &Vec<ast::Expr>) -> String {
    args.iter()
        .map(|arg| expression::generate(ctx, arg))
        .collect::<Vec<String>>()
        .join(", ")
}
