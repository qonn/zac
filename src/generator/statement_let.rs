use crate::ast;

use super::expression;

use super::context;
pub fn generate(ctx: &mut context::Context, ast: &ast::Let) -> String {
    let name = &ast.id;
    let expr = expression::generate(ctx, &ast.expr);
    ctx.add_var(name, &ast);

    format!("let {name} = {expr}")
}
