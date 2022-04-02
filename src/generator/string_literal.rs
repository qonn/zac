use crate::ast::AST;

pub fn generate(ast: &AST, jsx: bool) -> String {
    if let AST::StringLiteral { value, span: _ } = ast {
        if jsx {
            format!("{}", value.to_string())
        } else {
            format!("\"{}\"", value.to_string())
        }
    } else {
        "".into()
    }
}
