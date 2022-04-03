use lazy_static::__Deref;

use crate::{ast::AST, generator::expression};

pub fn generate(ast: &AST) -> String {
    if let AST::ReturnStatement { expr, span: _ } = ast {
        format!(
            "return {}",
            if let Some(expr) = expr {
                expression::generate(expr.deref())
            } else {
                "".into()
            }
        )
    } else {
        "".into()
    }
}
