use crate::{ast, generator::literal_string};

use super::context;
use super::expression;

pub fn generate(ctx: &mut context::Context, attr: &ast::JsxElementAttribute) -> String {
    let name = &attr.name;
    let ast_expr = &attr.expr;

    let expr = match ast_expr {
        ast::Expr::LitString(v) => literal_string::generate(ctx, &v, false, true),
        _ => generate_expression(ctx, &attr.expr),
    };

    format!("{name}={expr}")
}

pub fn generate_expression(ctx: &mut context::Context, expr: &ast::Expr) -> String {
    let expr = expression::generate(ctx, &expr);
    format!("{{{expr}}}")
}
