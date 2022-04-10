use crate::ast;

use super::{
    context, fn_call, literal_js, member_access, statement_fn, statement_let, statement_mod,
    statement_return,
};

pub fn generate(ctx: &mut context::Context, root_ast: &ast::Root) -> String {
    generate_statements(ctx, &root_ast.stmts)
        .join("\n\n")
        .trim()
        .to_string()
}

fn generate_statements(ctx: &mut context::Context, stmts: &Vec<ast::Stmt>) -> Vec<String> {
    stmts
        .iter()
        .filter(|stmt| match stmt {
            ast::Stmt::Noop => false,
            _ => true,
        })
        .map(|stmt| match stmt {
            ast::Stmt::Mod(v) => statement_mod::generate(ctx, v),
            ast::Stmt::Let(v) => statement_let::generate(ctx, v),
            ast::Stmt::Record(v) => "record_todo".into(),
            ast::Stmt::Fn(v) => statement_fn::generate(ctx, v),
            ast::Stmt::FnCall(v) => fn_call::generate(ctx, v),
            ast::Stmt::MemberAccess(v) => member_access::generate(ctx, v),
            ast::Stmt::Return(v) => statement_return::generate(ctx, v),
            ast::Stmt::LitJs(v) => literal_js::generate(ctx, v),
            ast::Stmt::Noop => "".into(),
            _ => panic!("Unexpected AST Node {:#?}", stmt),
        })
        .collect::<Vec<_>>()
}
