use crate::ast::{ASTKind, AST};

use super::{function_call, function_definition};

pub fn generate(root_ast: &AST) -> String {
    if let AST::Root { children } = root_ast {
        generate_children(children).join("\n\n").trim().to_string()
    } else {
        "".into()
    }
}

fn generate_children(children: &[AST]) -> Vec<String> {
    let mut lines = vec![];
    let mut children_iter = children.iter();

    while let Some(child) = children_iter.next() {
        match ASTKind::from(child) {
            ASTKind::TypeDefinition => {}
            ASTKind::FunctionCall => lines.push(function_call::generate(child)),
            ASTKind::FunctionDefinition => lines.push(function_definition::generate(child)),
            _ => panic!("Unexpected AST Node {:#?}", child),
        }
    }

    lines
}
