use crate::ast;

pub fn generate(ast: &ast::Ident) -> String {
    ast.string.to_string()
}
