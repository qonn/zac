use crate::ast;

pub fn generate(ast: &ast::LitBoolean) -> String {
    format!("{}", ast.value.to_string())
}
