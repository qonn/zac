use crate::ast::{self};

use super::{fn_call, literal_js, member_access, statement_if, statement_let, statement_return};

use super::context;
pub fn generate(ctx: &mut context::Context, ast: &ast::Fn) -> String {
    let id_ = ast.id.string.clone();

    let id = if ctx.module_path.len() > 0 {
        format!("{}_{}", ctx.module_path, id_)
    } else {
        id_
    };

    ctx.add_fn(&id, ast);

    format!(
        "function {}({}) {{{}}}",
        id,
        generate_args(ctx, &ast.args),
        generate_body(ctx, &ast.stmts)
    )
}

pub fn generate_args(ctx: &mut context::Context, args: &Vec<ast::FnArg>) -> String {
    let result = "";

    for arg in args.iter() {
        ctx.add_resolved_type(&arg.id.string, &arg.input.clone().into());
    }

    args.iter()
        .map(|arg| arg.id.string.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

pub fn generate_body(ctx: &mut context::Context, body: &Vec<ast::FnStmt>) -> String {
    let body = body
        .iter()
        .map(|fn_stmt| match fn_stmt {
            ast::FnStmt::Let(v) => statement_let::generate(ctx, v),
            ast::FnStmt::FnCall(v) => fn_call::generate(ctx, v),
            ast::FnStmt::MemberAccess(v) => member_access::generate(ctx, v),
            ast::FnStmt::LitJs(v) => literal_js::generate(ctx, v),
            ast::FnStmt::If(v) => statement_if::generate(ctx, v),
            ast::FnStmt::Ret(v) => statement_return::generate(ctx, v),
        })
        .collect::<Vec<String>>()
        .join("\n")
        .split("\n")
        .map(|x| format!("  {x}"))
        .collect::<Vec<String>>()
        .join("\n");

    let body = format!("\n{body}\n");

    body
}
