use crate::ast::AST;

pub fn generate(ast: &AST) -> String {
    if let AST::JsLiteral { value, span: _ } = ast {
        value.to_string()
    } else {
        "".into()
    }
}
