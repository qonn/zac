use crate::ast;

use super::context;
use super::statement_fn;

pub(crate) fn generate(ctx: &mut context::Context, module: &crate::ast::Mod) -> String {
    let path = module.path.replace(".", "_");

    let mod_ctx = &mut ctx.with_module_path(path.clone());

    let result = module
        .stmts
        .iter()
        .filter(|stmt| match stmt {
            ast::Stmt::Noop => false,
            _ => true,
        })
        .map(|stmt| match stmt {
            ast::Stmt::Fn(v) => statement_fn::generate(mod_ctx, v),
            ast::Stmt::Noop => "".into(),
            _ => ctx.throw_custom(stmt, "Unsupported statement while generating module"),
        })
        .collect::<Vec<_>>()
        .join("\n");

    ctx.merge(mod_ctx);
    ctx.add_mod(&module.path, &module);

    result
}
