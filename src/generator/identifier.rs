use crate::ast::AST;

pub fn generate(ast: &AST) -> String {
    match ast {
        AST::Identifier {
            value,
            generics: _,
            span: _,
        } => value.to_string(),
        _ => panic!("identifier.generate: Unexpected AST Node {ast:#?}"),
    }
}
